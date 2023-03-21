<template>
    <div class="rounded-md bg-red-50 p-4">
        <div class="flex">
            <div class="flex-shrink-0">
                <svg fill="none" stroke="currentColor" stroke-width="1.5" class="w-5 h-5 text-red-400" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" d="m9.75 9.75 4.5 4.5m0-4.5-4.5 4.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0z" />
                </svg>
            </div>
            <div class="ml-3">
                <h3 class="text-sm font-medium text-red-800">There were {{errors.length}} error(s) with your submission</h3>
                <div class="mt-2 text-sm text-red-700">
                    <ul role="list" class="list-disc space-y-1 pl-5">
                        <li v-for="error in errors">{{ error }}</li>
                    </ul>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
    const props = defineProps<{
        errors: string[];
    }>();
</script>