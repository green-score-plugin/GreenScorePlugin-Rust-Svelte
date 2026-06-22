from locust import HttpUser, task, between
import random
import uuid
import json

# Disable SSL warnings if testing against https with self-signed certs
import urllib3
urllib3.disable_warnings(urllib3.exceptions.InsecureRequestWarning)

class GreenScoreUser(HttpUser):
    wait_time = between(1, 5)

    def mark_expected_status(self, response, expected_statuses, action_name):
        """Mark request success only for explicit expected statuses; 5xx is always a failure."""
        if response.status_code in expected_statuses:
            response.success()
            return
        if response.status_code >= 500:
            response.failure(f"{action_name} server error {response.status_code}: {response.text[:180]}")
            return
        response.failure(f"{action_name} unexpected status {response.status_code}: {response.text[:180]}")

    def on_start(self):
        """Called when a User starts running."""
        self.user_id = None
        self.organisation_id = None
        self.service_ids = []
        self.organization_member_ids = []
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
                            self.organization_member_ids = []
                        services = data.get("services") or []
                        if isinstance(services, list):
                            self.service_ids = [service["id"] for service in services if "id" in service]
                except json.JSONDecodeError:
                    response.fail("Invalid JSON response from get-account")
            else:
                # expected if not logged in, but we should be
                response.fail(f"Get account failed: {response.status_code}")

    @task(3)
    def view_advice(self):
        with self.client.get("/home/advice", catch_response=True) as response:
            self.mark_expected_status(response, {200}, "view_advice")

    @task(3)
    def calculate_equivalent(self):
        # Public endpoint
        gco2 = random.uniform(50.0, 500.0)
        payload = {
            "gCO2": gco2,
            "count": 3
        }
        with self.client.post("/plugin/equivalent", json=payload, catch_response=True) as response:
            self.mark_expected_status(response, {200}, "calculate_equivalent")

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

        with self.client.post("/plugin/save_monitored_website_data", json=payload, catch_response=True) as response:
            self.mark_expected_status(response, {200}, "save_website_data")

    @task(2)
    def view_dashboard_pages(self):
        if not self.user_id:
            return

        # These are the protected pages
        with self.client.get("/mes-donnees", catch_response=True) as response:
            self.mark_expected_status(response, {200}, "view_mes_donnees")

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
        with self.client.get("/derniere-page-consultee", params=params, catch_response=True) as response:
            self.mark_expected_status(response, {200}, "view_derniere_page_consultee")

    @task(1)
    def view_organization(self):
        if not self.user_id:
            return

        # This will fail if user is not in organization, so catch 404/500
        with self.client.get("/mon-organisation", catch_response=True) as response:
            self.mark_expected_status(response, {200, 404}, "view_mon_organisation")

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

    @task(1)
    def logout_and_relogin(self):
        if not self.user_id:
            return

        with self.client.post("/auth/logout", catch_response=True) as response:
            if response.status_code == 200:
                response.success()
                self.user_id = None
                self.organisation_id = None
                self.service_ids = []
                self.login()
            elif response.status_code in [401, 403]:
                response.success()
            else:
                response.failure(f"Logout failed: {response.status_code}")

    @task(1)
    def update_account(self):
        if not self.user_id:
            return
        # Note: Backend uses 'prenom'/'nom' for update, but 'firstname'/'lastname' for inscription.
        payload = {
            "prenom": f"User{random.randint(1001, 2000)}",
            "nom": f"Updated{random.randint(1, 1000)}"
        }
        with self.client.patch("/account/update", json=payload, catch_response=True) as response:
            self.mark_expected_status(response, {200}, "update_account")

    @task(1)
    def delete_account(self):
        if not self.user_id:
            return
        # Expect success or redirect
        with self.client.delete("/account/delete", catch_response=True) as response:
            if response.status_code == 200:
                self.user_id = None # User deleted
                self.organisation_id = None
                self.service_ids = []
            else:
                 response.failure(f"Delete failed: {response.status_code}")

    @task(1)
    def join_organization(self):
        if not self.user_id:
            return
        payload = {"code": "TESTCODE"}
        with self.client.patch("/account/join-organization", json=payload, catch_response=True) as response:
             if response.status_code == 200:
                 response.success()
                 self.get_account_info() # Refresh to get org id
             else:
                 self.mark_expected_status(response, {400}, "join_organization")

    @task(1)
    def leave_organization(self):
        if not self.user_id or not self.organisation_id:
            return

        payload = {"organisationId": self.organisation_id}
        with self.client.post("/account/leave-organization", json=payload, catch_response=True) as response:
             if response.status_code == 200:
                 self.organisation_id = None
                 self.service_ids = []
             elif response.status_code == 400: # Not in org
                 response.success()
             else:
                 response.failure(f"Leave org failed: {response.status_code}")

    @task(1)
    def delete_organization(self):
        if not self.user_id or not self.organisation_id:
            return

        payload = {"organisationId": self.organisation_id}
        with self.client.post("/account/delete-organization", json=payload, catch_response=True) as response:
            if response.status_code == 200:
                response.success()
                self.organisation_id = None
                self.service_ids = []
            elif response.status_code in [400, 401, 403]:
                response.success()
            else:
                response.failure(f"Delete organization failed: {response.status_code}")

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
                 response.success()
                 self.get_account_info() # Refresh
             else:
                 self.mark_expected_status(response, {400}, "create_organization")

    @task(1)
    def create_service(self):
        if not self.user_id or not self.organisation_id:
            return

        payload = {
            "service_name": f"Service{random.randint(1, 10000)}",
            "organisation_id": self.organisation_id,
        }
        with self.client.post("/auth/create_service", json=payload, catch_response=True) as response:
            if response.status_code == 200:
                try:
                    data = response.json()
                    services = data.get("services") or []
                    if isinstance(services, list):
                        self.service_ids = [service["id"] for service in services if "id" in service]
                    response.success()
                except Exception:
                    response.failure("Failed to parse create service response")
            elif response.status_code in [400, 401, 403]:
                response.success()
            else:
                response.failure(f"Create service failed: {response.status_code}")

    @task(1)
    def get_organisation_members(self):
        if not self.user_id:
            return
        payload = {"organisation_id": self.organisation_id} if self.organisation_id else {}
        with self.client.post("/account/organization/members", json=payload, catch_response=True) as response:
            if response.status_code == 200:
                response.success()
                try:
                    data = response.json()
                    members = data.get("members") or []
                    if isinstance(members, list):
                        self.organization_member_ids = [m["id"] for m in members if m.get("id") != self.user_id]
                except Exception:
                    self.organization_member_ids = []
            else:
                self.mark_expected_status(response, {400, 401, 403}, "get_organisation_members")

    @task(1)
    def get_organisation_services(self):
        if not self.user_id:
            return

        payload = {"organisation_id": self.organisation_id} if self.organisation_id else {}
        with self.client.post("/account/organization/services", json=payload, catch_response=True) as response:
            if response.status_code == 200:
                try:
                    data = response.json()
                    services = data.get("services") or []
                    if isinstance(services, list):
                        self.service_ids = [service["id"] for service in services if "id" in service]
                    response.success()
                except Exception:
                    response.failure("Failed to parse services response")
            elif response.status_code in [400, 401, 403]:
                response.success()
            else:
                response.failure(f"Get services failed: {response.status_code}")

    @task(1)
    def remove_organisation_member(self):
        if not self.user_id or not self.organisation_id or not self.organization_member_ids:
            return
        payload = {
            "userId": random.choice(self.organization_member_ids),
            "organisationId": self.organisation_id,
        }
        with self.client.post("/account/organization/members/remove", json=payload, catch_response=True) as response:
             if response.status_code == 200:
                 response.success()
                 self.get_organisation_members()
             else:
                 self.mark_expected_status(response, {400, 401, 403}, "remove_organisation_member")

    @task(1)
    def assign_user_to_service(self):
        if not self.user_id or not self.organisation_id or not self.service_ids or not self.organization_member_ids:
            return

        payload = {
            "serviceId": random.choice(self.service_ids),
            "userId": random.choice(self.organization_member_ids),
            "organisationId": self.organisation_id,
        }
        with self.client.post("/account/organization/services/assign", json=payload, catch_response=True) as response:
            self.mark_expected_status(response, {200, 400, 401, 403}, "assign_user_to_service")

    @task(1)
    def unassign_user_from_service(self):
        if not self.user_id or not self.organisation_id or not self.organization_member_ids:
            return

        payload = {
            "userId": random.choice(self.organization_member_ids),
            "organisationId": self.organisation_id,
        }
        with self.client.post("/account/organization/services/unassign", json=payload, catch_response=True) as response:
            self.mark_expected_status(response, {200, 400, 401, 403}, "unassign_user_from_service")

    @task(1)
    def delete_service(self):
        if not self.user_id or not self.organisation_id or not self.service_ids:
            return

        service_id = random.choice(self.service_ids)
        payload = {
            "serviceId": service_id,
            "organisationId": self.organisation_id,
        }
        with self.client.post("/account/organization/services/delete", json=payload, catch_response=True) as response:
            if response.status_code == 200:
                self.service_ids = [sid for sid in self.service_ids if sid != service_id]
                response.success()
            elif response.status_code in [400, 401, 403]:
                response.success()
            else:
                response.failure(f"Delete service failed: {response.status_code}")

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
        with self.client.post("/plugin/get-account", catch_response=True) as response:
            self.mark_expected_status(response, {200}, "plugin_get_account")
