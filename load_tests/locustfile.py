from locust import HttpUser, task, between, events
import random
import uuid
import json

# Disable SSL warnings if testing against https with self-signed certs
import urllib3
urllib3.disable_warnings(urllib3.exceptions.InsecureRequestWarning)

class GreenScoreUser(HttpUser):
    wait_time = between(1, 5)

    def on_start(self):
        """Called when a User starts running."""
        self.user_id = None
        self.organisation_id = None
        self.equivalent_ids = []
        self.email = f"user_{uuid.uuid4()}@example.com"
        self.password = "password123"
        self.firstname = f"User{random.randint(1, 1000)}"
        self.lastname = "Tester"

        # Register and Login
        self.register_and_login()

    def register_and_login(self):
        # 1. Registration (which also logs in based on backend code)
        payload = {
            "email": self.email,
            "password": self.password,
            "firstname": self.firstname,
            "lastname": self.lastname
        }

        with self.client.post("/auth/inscription", json=payload, catch_response=True) as response:
            if response.status_code == 200:
                response.success()
            else:
                # If registration fails (e.g. user exists in a long run), try login
                self.login()
                return

        # 2. Get User ID
        self.get_account_info()

    def login(self):
        payload = {
            "email": self.email,
            "password": self.password
        }
        with self.client.post("/auth/login", json=payload, catch_response=True) as response:
            if response.status_code == 200:
                self.get_account_info()
            else:
                response.fail(f"Login failed: {response.text}")

    def get_account_info(self):
        with self.client.post("/auth/get-account", catch_response=True) as response:
            if response.status_code == 200:
                try:
                    data = response.json()
                    if data.get("success") and data.get("user_full"):
                        self.user_id = data["user_full"]["user"]["id"]
                        if data["user_full"].get("organisation") and len(data["user_full"]["organisation"]) > 0:
                            self.organisation_id = data["user_full"]["organisation"][0]["id"]
                        else:
                            self.organisation_id = None
                except json.JSONDecodeError:
                    response.fail("Invalid JSON response from get-account")
            else:
                # expected if not logged in, but we should be
                response.fail(f"Get account failed: {response.status_code}")

    @task(3)
    def view_advice(self):
        self.client.get("/home/advice")

    @task(3)
    def calculate_equivalent(self):
        # Public endpoint
        gco2 = random.uniform(50.0, 500.0)
        payload = {
            "gCO2": gco2,
            "count": 3
        }
        self.client.post("/plugin/equivalent", json=payload)

    @task(5)
    def save_website_data(self):
        if not self.user_id:
            return

        # Simulate plugin saving data
        domain = f"example{random.randint(1, 100)}.com"
        url = f"https://{domain}/page{random.randint(1, 10)}"

        payload = {
            "id": 0, # Ignored/Auto-increment
            "user_id": self.user_id,
            "url_domain": domain,
            "url_full": url,
            "queries_quantity": random.randint(10, 50),
            "data_transferred": random.randint(100000, 5000000),
            "resources": random.randint(5, 20),
            "loading_time": random.uniform(0.5, 3.0),
            "carbon_footprint": random.uniform(0.1, 2.0),
            "country": "FR"
        }

        self.client.post("/plugin/save_monitored_website_data", json=payload)

    @task(2)
    def view_dashboard_pages(self):
        if not self.user_id:
            return

        # These are the protected pages
        self.client.get("/mes-donnees")

        # Last Page Consulted (requires query params usually)
        # Note: /derniere-page-consultee maps to lpc_controller::lpc
        # It expects Query Params: LastPageConsultedInfos struct.
        params = {
            "url_full": "https://google.com",
            "queries_quantity": 10,
            "carbon_footprint": 1.5,
            "data_transferred": 102400,
            "loading_time": 1.2,
            "country": "US"
        }
        self.client.get("/derniere-page-consultee", params=params)

    @task(1)
    def view_organization(self):
        if not self.user_id:
            return

        # This will fail if user is not in organization, so catch 404/500
        with self.client.get("/mon-organisation", catch_response=True) as response:
            if response.status_code == 404: # "User is not in an organization"
                response.success()
            elif response.status_code == 200:
                response.success()

    @task(2)
    def view_equivalents(self):
        if not self.user_id:
            return

        with self.client.get("/account/equivalents", catch_response=True) as response:
            if response.status_code == 200:
                try:
                    data = response.json()
                    if data.get("success") and data.get("equivalents"):
                        self.equivalent_ids = [eq["id"] for eq in data["equivalents"]]
                    response.success()
                except Exception:
                   response.failure("Failed to parse equivalents")
            else:
                response.failure(f"Get equivalents failed: {response.status_code}")

    # Logout removed to prevent session invalidation during test loop
    # @task(1)
    # def logout(self):
    #    self.client.post("/auth/logout")

    @task(1)
    def update_account(self):
        if not self.user_id:
            return
        # Note: Backend uses 'prenom'/'nom' for update, but 'firstname'/'lastname' for inscription.
        payload = {
            "prenom": f"User{random.randint(1001, 2000)}",
            "nom": f"Updated{random.randint(1, 1000)}"
        }
        self.client.patch("/account/update", json=payload)

    @task(1)
    def delete_account(self):
        if not self.user_id:
            return
        # Expect success or redirect
        with self.client.delete("/account/delete", catch_response=True) as response:
            if response.status_code == 200:
                self.user_id = None # User deleted
            else:
                 response.failure(f"Delete failed: {response.status_code}")

    @task(1)
    def join_organization(self):
        if not self.user_id:
            return
        payload = {"code": "TESTCODE"}
        with self.client.patch("/account/join-organization", json=payload, catch_response=True) as response:
             if response.status_code == 400: # Invalid code or already joined
                 response.success()
             elif response.status_code == 200:
                 self.get_account_info() # Refresh to get org id

    @task(1)
    def leave_organization(self):
        if not self.user_id or not self.organisation_id:
            return

        payload = {"organisationId": self.organisation_id}
        with self.client.post("/account/leave-organization", json=payload, catch_response=True) as response:
             if response.status_code == 200:
                 self.organisation_id = None
             elif response.status_code == 400: # Not in org
                 response.success()
             else:
                 response.failure(f"Leave org failed: {response.status_code}")

    @task(1)
    def my_organization(self):
        if not self.user_id:
            return
        # Expect 404 if no org
        with self.client.get("/account/my-organization", catch_response=True) as response:
            if response.status_code == 404 and not self.organisation_id:
                response.success()
            elif response.status_code == 200:
                response.success()

    @task(1)
    def create_organization(self):
        if not self.user_id:
            return
        payload = {"organization_name": f"Org{random.randint(1, 10000)}", "siret": str(random.randint(10000000000000, 99999999999999))}
        with self.client.post("/account/organization/create", json=payload, catch_response=True) as response:
             if response.status_code == 200:
                 self.get_account_info() # Refresh
             elif response.status_code == 400: # Siret exists
                 response.success()

    @task(1)
    def get_organisation_members(self):
        if not self.user_id:
            return
        payload = {"organisation_id": self.organisation_id} if self.organisation_id else {}
        with self.client.post("/account/organization/members", json=payload, catch_response=True) as response:
            if response.status_code == 200:
                response.success()
            elif response.status_code in [401, 403, 400]: # Not in org or unauthenticated logic
                response.success()
            else:
                response.failure(f"Get members failed: {response.status_code}")

    @task(1)
    def remove_organisation_member(self):
        if not self.user_id or not self.organisation_id:
            return
        payload = {"userId": self.user_id, "organisationId": self.organisation_id}
        # Expect error when removing self
        with self.client.post("/account/organization/members/remove", json=payload, catch_response=True) as response:
             if response.status_code in [400, 500]: # Cannot remove self
                 response.success()

    @task(1)
    def update_organisation(self):
        if not self.user_id:
            return
        payload = {"name": f"OrgUpdated{random.randint(1, 10000)}"}
        with self.client.patch("/account/organization/update", json=payload, catch_response=True) as response:
             if response.status_code == 200:
                 response.success()
             elif response.status_code in [401, 403]: # Not admin or not in org
                 response.success()
             elif response.status_code == 400: # Name exists or invalid, handled gracefully
                 response.success()
             else:
                 response.failure(f"Update org failed: {response.status_code}")

    @task(1)
    def patch_equivalents(self):
        if not self.user_id:
            return

        ids_to_send = []
        if hasattr(self, 'equivalent_ids') and self.equivalent_ids:
             count = min(len(self.equivalent_ids), 3)
             ids_to_send = random.sample(self.equivalent_ids, count)

        if not ids_to_send:
             return

        payload = {"equivalents": ids_to_send}
        with self.client.patch("/account/equivalents", json=payload, catch_response=True) as response:
             if response.status_code == 200:
                 response.success()
             else:
                 response.failure(f"Patch equivalents failed: {response.status_code}")

    @task(1)
    def plugin_get_account(self):
        if not self.user_id:
            return
        self.client.post("/plugin/get-account")
