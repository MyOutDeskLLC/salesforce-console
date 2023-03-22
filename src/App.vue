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
                <ul class="tracking-wider divide-gray-400 divide-y" style="font-size: 0.6rem">
                    <li v-for="recentQuery in recentQueries" class="cursor-pointer truncate px-1 py-1 hover:bg-sf-100" :class="[selectedQuery === recentQuery ? 'bg-sf-100' : '']" @click="changeQuery(recentQuery)">
                        {{ recentQuery }}
                    </li>
                </ul>
            </div>
            <div class="flex max-h-screen grow flex-col overflow-auto border bg-gray-100 overflow-none">
                <div class="h-64">
                    <ErrorAlert v-if="errors.length > 0" :errors="errors" />
                    <div>
                        <textarea v-model="queryString" rows="8" class="block w-full resize-none border-0 text-xs placeholder:text-gray-200 text-gray-900 py-1.5 focus:ring-0" />
                        <div class="bg-gray-100 py-4">
                            <div class="flex justify-center text-center">
                                <div class="ml-4 flex items-center justify-center text-xs text-gray-500">
                                    <MacLogoSvg class="h-4 w-4" />
                                    <div class="ml-1">Command + Enter</div>
                                </div>
                                <div class="ml-4 flex items-center justify-center text-xs text-gray-500">
                                    <WindowsLogoSvg class="h-4 w-4" />
                                    <div class="ml-1">Control + Enter</div>
                                </div>
                            </div>
                            <div class="mt-2 flex justify-center">
                                <button class="rounded bg-blue-500 px-4 py-1 text-xs text-white disabled:cursor-not-allowed disabled:bg-opacity-50" @click="submit" :disabled="disableSubmit">
                                    <span class="flex items-center">
                                        <Spinner class="h-3 w-3 text-white" v-if="disableSubmit" />
                                        <span>Query</span>
                                    </span>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="flex w-full items-center px-2 mb-2">
                    <div>
                        <button @click="copyData" :disabled="tableRowData.length < 1" class="flex items-center rounded bg-white p-1 py-2 text-xs space-x-1 px-3.5 disabled:cursor-not-allowed disabled:opacity-50">
                            <span>Copy</span>
                            <ClipboardCopySvg class="h-4 w-4" />
                        </button>
                    </div>
                    <div class="shrink-0 grow text-right text-xs self-end">
                        Showing {{ tableRowData.length }} Row(s)
                    </div>
                </div>
                <div class="max-w-full overflow-x-auto overflow-y-auto bg-white">
                    <table class="min-w-full divide-y divide-gray-300">
                        <thead>
                            <tr>
                                <th scope="col" class="pr-3 pl-4 text-left text-xs font-semibold text-gray-900 py-3.5" v-for="header in tableHeaders">{{ header }}</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-gray-200">
                            <tr class="odd:bg-gray-50 hover:bg-sf-50" v-for="row in tableRowData">
                                <td class="whitespace-nowrap py-2 pr-3 pl-4 text-xs font-medium text-gray-900" v-for="value in row">{{ value }}</td>
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
    import MacLogoSvg from './components/MacLogoSvg.vue';
    import WindowsLogoSvg from './components/WindowsLogoSvg.vue';
    import Spinner from './components/Spinner.vue';
    import clipboard from 'clipboardy';
    import ClipboardCopySvg from './components/ClipboardCopySvg.vue';

    const loggedIn = ref(false);
    const disableSubmit = ref(false);
    const errors = ref<string[]>([]);
    const queryString = ref('');
    const recentQueries = ref<string[]>([]);
    const tableHeaders = ref<string[]>([]);
    const tableRowData = ref<string[]>([]);
    const db = new Localbase('db');
    const selectedQuery = ref('');

    function handleSuccessfulLogin() {
        loggedIn.value = true;
    }

    function handleLogout() {
        loggedIn.value = false;
    }

    function changeQuery(query: string) {
        selectedQuery.value = queryString.value = query;
    }

    async function submit() {
        //if disabled just return
        if (disableSubmit.value) {
            return;
        }

        disableSubmit.value = true;
        try {
            queryString.value = formatQuery(queryString.value, {
                fieldMaxLineLength: 20,
                fieldSubqueryParensOnOwnLine: true,
                whereClauseOperatorsIndented: true,
            });
        } catch (error: any) {
            console.error(error);
            disableSubmit.value = false;
            errors.value = [];
            errors.value.push(error.message);
            return;
        }
        errors.value = [];
        let typeOfQuery = queryString.value.split(' ')[0].toUpperCase();
        switch (typeOfQuery) {
            case 'SELECT':
                await query();
                break;
            case 'UPDATE':
                break;
            case 'INSERT':
                break;
        }
        disableSubmit.value = false;
    }

    interface GenericObject {
        [key: string]: any;
    }

    function query(): Promise<void> {
        storeQuery(queryString.value);
        return invoke<string>('query', { query: queryString.value })
            .then((response: string) => {
                //reset table data
                tableHeaders.value = tableRowData.value = [];
                //parse the JSON from Rust
                let data: GenericObject = JSON.parse(response);

                //delete the attributes from each row
                //@ts-ignore
                data = data.map(({ attributes, ...rest }) => {
                    return rest;
                });

                let rowArray: string[] = [];
                let rowObjects: string[] = [];
                //Get the table headers from the first row
                Object.keys(data[0]).forEach((key: string) => {
                    if (typeof data[0][key] === 'object') {
                        rowObjects.push(key);
                    } else {
                        rowArray.push(key);
                    }
                });
                tableHeaders.value = rowArray.concat(rowObjects);

                for (let i in data) {
                    let rowArray: any[] = [];
                    let rowObjects: GenericObject[] = [];
                    Object.values(data[i]).forEach((value: any) => {
                        if (!value) {
                            return;
                        }
                        if (typeof value === 'object') {
                            //@ts-ignore
                            let records = value.records.map(({ attributes, ...rest }) => rest);
                            rowObjects.push(records);
                        } else {
                            rowArray.push(value);
                        }
                    });
                    //merge two arrays
                    tableRowData.value.push(rowArray.concat(rowObjects));
                }

                //Get the table row data from the rest of the rows
                // data.forEach((row: GenericObject) => {
                //     let rowValues: any[] = Object.values(row);
                //
                //     Object.keys(subQueryData).forEach((key: string) => {
                //         rowValues.push(subQueryData[key]);
                //     });
                //     //@ts-ignore
                //     tableRowData.value.push(rowValues);
                // });
            })
            .catch((error) => {
                console.error(error);
                errors.value = [];
                let parsed: [] = JSON.parse(error);
                parsed.forEach((error: any) => {
                    errors.value.push(error.message);
                });
            })
            .finally(() => {
                return getQueries();
            });
    }

    function registerWindowEventListeners() {
        window.addEventListener('keydown', handleKeyDown, { capture: true });
    }

    function handleKeyDown(e: KeyboardEvent) {
        if (e.metaKey || e.ctrlKey) {
            if (e.code === 'KeyR') {
                e.preventDefault();
                window.location.reload();
                return;
            }
            if (e.code === 'Enter') {
                e.preventDefault();
                submit();
                return;
            }
        }
    }

    function doesQueryExistInStorage(query: string) {
        return db
            .collection('queries')
            .doc({ id: btoa(query) })
            .get()
            .then((data: StoredQuery[]) => {
                return data && data.length > 0;
            });
    }

    async function storeQuery(query: string): Promise<void> {
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

    interface StoredQuery {
        query: string;
        updated_at: string;
        created_at: string;
        id: string;
    }

    function getQueries(): Promise<void> {
        return db
            .collection('queries')
            .get()
            .then((data: StoredQuery[]) => {
                recentQueries.value = [];
                data.reverse();
                data.forEach((datum: StoredQuery) => {
                    recentQueries.value.push(datum.query);
                });
            });
    }

    async function copyData() {
        let data: string[] = [];
        data.push(tableHeaders.value.join(','));
        tableRowData.value.forEach((row: any[]) => {
            let rd: any = [];
            row.forEach((value: string | object) => {
                if (typeof value === 'object') {
                    rd.push(JSON.stringify(value));
                } else {
                    rd.push(value);
                }
            });
            data.push(rd.join(','));
        });
        console.log(data)
        await clipboard.write(data.toString());
    }

    onMounted(async () => {
        await getQueries();
        // getRecentQueries();
        registerWindowEventListeners();
    });
</script>