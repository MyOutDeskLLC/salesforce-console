<template>
    <Login v-if="!loggedIn" @success="handleSuccessfulLogin"></Login>
    <div v-else>
        <div class="flex max-h-screen overflow-none">
            <div class="min-h-screen w-48 shrink-0 bg-gray-100">
                <div class="flex items-center border-b px-2 py-2 font-semibold">
                    <div class="grow">Recent Queries</div>
                    <div>
                        <button class="hover:text-gray-500" @click="handleLogout">
                            <LogoutSvg class="" />
                        </button>
                    </div>
                </div>
                <ul class="text-xs divide-gray-400 divide-y">
                    <li v-for="recentQuery in recentQueries" class="truncate px-1 py-2" @click="changeQuery(recentQuery)">
                        {{ recentQuery }}
                    </li>
                </ul>
            </div>
            <div class="max-h-screen grow overflow-auto border spacey-y-4 overflow-none">
                <ErrorAlert class="mb-4" v-if="errors.length > 0" :errors="errors" />
                <div>
                    <textarea v-model="queryString" rows="8" class="block w-full resize-none border-0 text-xs text-gray-900 placeholder:text-gray-40 py-1.5 focus:ring-0" placeholder="SELECT Id from Account WHERE created_at > 2022-01-01T00:00:00.0000Z" />
                    <div class="mt-1 text-center">
                        <button class="bg-blue-500 px-4 py-1 text-xs text-white rounded disabled:cursor-not-allowed disabled:bg-opacity-50" @click="submit" :disabled="disabled">Query</button>
                        <span class="ml-4 text-xs text-gray-500">* Hot Key Command + Enter</span>
                    </div>
                </div>
                <div class="max-w-full overflow-auto">
                    <table class="min-w-full divide-y divide-gray-300">
                        <thead>
                            <tr>
                                <th scope="col" class="pr-3 pl-4 text-left text-xs font-semibold text-gray-900 py-3.5" v-for="header in tableHeaders">{{ header }}</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-gray-200">
                            <tr v-for="row in tableRowData">
                                <td class="py-2 pr-3 pl-4 text-xs font-medium text-gray-900 hitespace-nowrap" v-for="value in row">{{ value }}</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
    import { onMounted, ref } from 'vue';
    import Login from './components/Login.vue';
    import { invoke } from '@tauri-apps/api/tauri';
    import ErrorAlert from './components/ErrorAlert.vue';
    import LogoutSvg from './components/LogoutSvg.vue';
    import { formatQuery } from 'soql-parser-js';
    import Localbase from 'localbase';

    const loggedIn = ref(false);
    const disabled = ref(false);
    const errors = ref<string[]>([]);
    const queryString = ref('');
    const recentQueries = ref<string[]>([]);
    const tableHeaders = ref<string[]>([]);
    const tableRowData = ref<string[]>([]);
    const db = new Localbase('db');
    const records = ref(0)

    function handleSuccessfulLogin() {
        loggedIn.value = true;
    }

    function handleLogout() {
        loggedIn.value = false;
    }

    function changeQuery(query: string) {
        queryString.value = query;
    }

    async function submit() {
        disabled.value = true;
        try {
            queryString.value = formatQuery(queryString.value, {
                fieldMaxLineLength: 20,
                fieldSubqueryParensOnOwnLine: true,
                whereClauseOperatorsIndented: true,
            });
        } catch (error) {
            errors.value = [];
            errors.value.push(error.message);
            disabled.value = false;
            return;
        }
        errors.value = [];
        let typeOfQuery = queryString.value.split(' ')[0].toUpperCase();
        switch (typeOfQuery) {
            case 'SELECT':
                query();
                break;
            case 'UPDATE':
                break;
            case 'INSERT':
                break;
        }
        disabled.value = false;
    }

    function query() {
        storeQuery(queryString.value);
        invoke('query', { query: queryString.value })
            .then((response) => {
                //reset table data
                tableHeaders.value = tableRowData.value = [];
                //parse the JSON from Rust
                let data = JSON.parse(response);

                //delete the attributes from each row
                data = data.map(({ attributes, ...rest }) => rest);
                //Get the table headers from the first row
                tableHeaders.value = Object.keys(data[0]);
                //Get the table row data from the rest of the rows
                data.forEach((row) => {
                    tableRowData.value.push(Object.values(row));
                });
            })
            .catch((error) => {
                errors.value = [];
                let parsed: Object = JSON.parse(error);
                parsed.forEach((error: any) => {
                    errors.value.push(error.message);
                });
            })
            .finally(() => {
                getQueries();
            });
    }

    function registerWindowEventListeners() {
        window.addEventListener('keydown', handleKeyDown, { capture: true });
    }

    function handleKeyDown(e: KeyboardEvent) {
        if (e.metaKey && e.code === 'KeyR') {
            e.preventDefault();
            window.location.reload();
            return;
        }
        if (e.metaKey && e.code === 'Enter') {
            e.preventDefault();
            //if its disabled, don't do anything
            if (disabled.value) {
                return;
            }
            submit();
            return;
        }
    }

    function doesQueryExistInStorage(query) {
        return db
            .collection('queries')
            .doc({ id: btoa(query) })
            .get()
            .then((data) => {
                return data.length > 0;
            });
    }

    async function storeQuery(query): Promise<void> {
        //if it already exists don't store it
        if (await doesQueryExistInStorage(query)) {
            return;
        }
        return db
            .collection('queries')
            .doc({ id: btoa(query) })
            .update({ query, updated_at: new Date().toISOString() })
            .catch(() => {
                return db.collection('queries').add({
                    id: btoa(query),
                    query,
                    created_at: new Date().toISOString(),
                    updated_at: new Date().toISOString(),
                });
            });
    }

    function getQueries(): Promise<SfLogin[]> {
        db.collection('queries')
            .get()
            .then((data) => {
                recentQueries.value = [];
                data.forEach((datum) => {
                    recentQueries.value.push(datum.query);
                });
            });
    }

    onMounted(async () => {
        await getQueries();
        // getRecentQueries();
        registerWindowEventListeners();
    });
</script>
