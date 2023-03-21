<template>
    <div class="flex min-h-full flex-col justify-center py-12 sm:px-6 lg:px-8">
        <div class="sm:mx-auto sm:w-full sm:max-w-md">
            <SalesforceSvg class="mx-auto h-12 w-auto"></SalesforceSvg>
            <h2 class="mt-6 text-center text-3xl font-bold tracking-tight text-gray-900">Sign in to your account</h2>
        </div>

        <div class="sm:mx-auto sm:w-full sm:max-w-md mt-6">
            <ErrorAlert :errors="errors" v-if="errors.length > 0"></ErrorAlert>
        </div>


        <div class="mt-6 sm:mx-auto sm:w-full sm:max-w-md">
            <div class="bg-white px-4 py-8 shadow sm:rounded-lg sm:px-10">
                <div v-if="loading" class="justify-center flex">
                    <Spinner></Spinner>
                </div>
                <div v-else-if="!loading && savedUsers.length > 0">
                    <button v-for="user in savedUsers" class="mb-1 w-full border bg-gray-50 p-4 hover:bg-gray-100 rounded-sm" @click="selectAndLogin(user)">
                        <span class="flex w-full">
                            <span class="grow text-left text-sm">{{ user.email }}</span>
                            <span class="">
                                <button type="button" class="flex min-h-full w-10 items-center justify-center hover:text-red-500" @click.stop="deleteUser(user.email)">
                                    <TrashSvg class="h-4 w-4"></TrashSvg>
                                </button>
                            </span>
                        </span>
                    </button>
                </div>
                <form class="space-y-6" v-else>
                    <div>
                        <label for="email" class="block text-sm font-medium leading-6 text-gray-900">Email address</label>
                        <div class="mt-2">
                            <input v-model="email" id="email" name="email" type="email" autocomplete="email" required class="block w-full rounded-md border-0 placeholder:text-gray-400 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 py-1.5 focus:ring-sf-600 focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6" />
                        </div>
                    </div>

                    <div>
                        <label for="password" class="block text-sm font-medium leading-6 text-gray-900">Password</label>
                        <div class="mt-2">
                            <input v-model="password" id="password" name="password" type="password" autocomplete="current-password" required class="block w-full rounded-md border-0 placeholder:text-gray-400 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 py-1.5 focus:ring-sf-600 focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6" />
                        </div>
                    </div>
                    <div>
                        <label for="client_id" class="block text-sm font-medium leading-6 text-gray-900">Client ID</label>
                        <div class="mt-2">
                            <input v-model="clientId" id="client_id" name="client_id" type="text" autocomplete="client_id" required class="block w-full rounded-md border-0 placeholder:text-gray-400 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 py-1.5 focus:ring-sf-600 focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6" />
                        </div>
                    </div>
                    <div>
                        <label for="client_secret" class="block text-sm font-medium leading-6 text-gray-900">Client Secret</label>
                        <div class="mt-2">
                            <input v-model="clientSecret" id="client_secret" name="client_secret" type="text" autocomplete="client_secret" required class="block w-full rounded-md border-0 placeholder:text-gray-400 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 py-1.5 focus:ring-sf-600 focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6" />
                        </div>
                    </div>

                    <div>
                        <button type="button" class="flex w-full justify-center rounded-md px-3 py-2 text-sm font-semibold text-white shadow-sm bg-sf-600 hover:bg-sf-500 focus-visible:outline-sf-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2" @click="login">Sign in</button>
                    </div>
                </form>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';

    import SalesforceSvg from './SalesforceSvg.vue';
    import Localbase from 'localbase';
    import { onMounted, ref, watch } from 'vue';
    import TrashSvg from './TrashSvg.vue';
    import ErrorAlert from './ErrorAlert.vue';
    import Spinner from './Spinner.vue';

    const db = new Localbase('db');

    const email = ref<string>('');
    const password = ref<string>('');
    const clientId = ref<string>('');
    const clientSecret = ref<string>('');
    const savedUsers = ref<SfLogin[]>([]);
    const selectedUser = ref<SfLogin>({ email: '', password: '', clientId: '', clientSecret: '' });
    const errors = ref<String[]>([])
    const loading = ref(false)
    const emits = defineEmits(["success"])

    watch(selectedUser, (user: SfLogin) => {
        handleSelectedUserChange(user);
    });

    function handleSelectedUserChange(user: SfLogin) {
        if (user) {
            email.value = user.email;
            password.value = user.password;
            clientId.value = user.clientId;
            clientSecret.value = user.clientSecret;
        }
    }

    function selectAndLogin(user: SfLogin){
        selectedUser.value = user;
        email.value = selectedUser.value.email;
        password.value = selectedUser.value.password;
        clientId.value = selectedUser.value.clientId;
        clientSecret.value = selectedUser.value.clientSecret;
        login()
    }

    async function login() {
        if (!email.value || !password.value || !clientId.value || !clientSecret.value) {
            return;
        }

        loading.value = true;

        let user: SfLogin = {
            email: email.value,
            password: password.value,
            clientId: clientId.value,
            clientSecret: clientSecret.value,
        };

        invoke("login", {username: email.value, password: password.value, clientId: clientId.value, clientSecret: clientSecret.value}).then( async (response)=>{
            errors.value = [];
            await storeUser(user);
            savedUsers.value = await getUsers();
            emits("success")
        }).catch(error=>{
            errors.value = [];
            errors.value.push(error)
        }).finally(()=>{
            loading.value = false;
        })
    }

    function getUsers(): Promise<SfLogin[]> {
        return db.collection('users').get();
    }

    function storeUser(user: SfLogin): Promise<void> {
        return db
            .collection('users')
            .doc({ id: user.email })
            .update(user)
            .catch(() => {
                return db.collection('users').add({ id: user.email, ...user });
            });
    }

    async function deleteUser(email: string): Promise<void> {
        await db.collection('users').doc({ id: email }).delete();
        savedUsers.value = await getUsers();
    }

    onMounted(() => {
        getUsers().then((users) => {
            savedUsers.value = [];
            if (users.length > 0) {
                savedUsers.value = users;
                selectedUser.value = users[0];
            }
        });
    });
</script>
