<script>
    let formData = {
        firstName: '',
        lastName: '',
        email: '',
        plainPassword: '',
        passwordConfirmation: ''
    };

    const handleSubmit = (event) => {
        event.preventDefault();
        console.log("Formulaire MyInfo :", formData);
    };
</script>

<h1 class="text-2xl font-bold py-2">Mes informations</h1>

<form on:submit={handleSubmit} class="flex flex-col gap-4">

    <!-- Prénom -->
    <div class="flex gap-4 w-full text-grey-700 font-outfit font-semibold text-sm">
        <div class="w-full flex flex-col">
            <label for="firstName">Prénom</label>
            <input
                    id="firstName"
                    type="text"
                    bind:value={formData.firstName}
                    class="px-4 py-2 border border-grey-200 rounded-lg text-grey-700 w-full focus:outline-none"
                    placeholder="Votre prénom"
            />
        </div>
    </div>

    <!-- Nom -->
    <div class="flex gap-4 w-full text-grey-700 font-outfit font-semibold text-sm">
        <div class="w-full flex flex-col">
            <label for="lastName">Nom</label>
            <input
                    id="lastName"
                    type="text"
                    bind:value={formData.lastName}
                    class="px-4 py-2 border border-grey-200 rounded-lg text-grey-700 w-full focus:outline-none"
                    placeholder="Votre nom"
            />
        </div>
    </div>

    <!-- Email -->
    <div class="flex gap-4 w-full text-grey-700 font-outfit font-semibold text-sm">
        <div class="w-full flex flex-col">
            <label for="email">Email</label>
            <input
                    id="email"
                    type="email"
                    bind:value={formData.email}
                    class="px-4 py-2 border border-grey-200 rounded-lg text-grey-700 w-full focus:outline-none"
                    placeholder="Votre email"
            />
        </div>
    </div>

    <!-- Mot de passe + Confirmation -->
    <div class="flex flex-col md:flex-row gap-4 w-full text-grey-700 font-outfit font-semibold text-sm">
        <div class="w-full flex flex-col">
            <label for="plainPassword">Mot de passe</label>
            <input
                    id="plainPassword"
                    type="password"
                    bind:value={formData.plainPassword}
                    class="px-4 py-2 border border-grey-200 rounded-lg text-grey-700 w-full focus:outline-none"
                    placeholder="Votre mot de passe"
            />
        </div>

        <div class="w-full flex flex-col">
            <label for="passwordConfirmation">Confirmation</label>
            <input
                    id="passwordConfirmation"
                    type="password"
                    bind:value={formData.passwordConfirmation}
                    class="px-4 py-2 border border-grey-200 rounded-lg text-grey-700 w-full focus:outline-none"
                    placeholder="Confirmez votre mot de passe"
            />
        </div>
    </div>

    <button
            type="submit"
            class="w-full h-fit rounded-lg bg-gs-green-950 px-1 py-2 font-semibold font-outfit text-white
               cursor-pointer hover:bg-gs-green-800 active:bg-gs-green-700
               transition-colors duration-150 ease-in-out"
    >
        Valider
    </button>
</form>
