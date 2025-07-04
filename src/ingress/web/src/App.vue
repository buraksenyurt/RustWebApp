<script setup lang="ts">
import {ref, onMounted} from "vue";
import getAll from "./api/get";
import type {WorkItems} from './interfaces/workItems';
import WorkItemTable from "./components/WorkItemTable.vue";

const data = ref<WorkItems | null>(null);
const loading = ref(true);
const error = ref<string | null>(null);

onMounted(async () => {
  try {
    const result = await getAll();

    if (result
        && typeof result === 'object'
        && 'data' in result) {
      data.value = result.data;
    } else if (result && 'error' in result) {
      error.value = result.error ?? 'Unknown error';
    } else {
      error.value = 'Unexpected API response';
    }
  } catch (e) {
    error.value = 'Unexpected error occurred while fetching data';
    console.error(e);
  } finally {
    loading.value = false;
  }
});
</script>

<template>
  <div>
    <h2>Work Item Dashboard</h2>
    <p v-if="loading">Loading</p>
    <p v-else-if="error">Error: {{ error }}</p>
    <div v-else>
      <WorkItemTable title="Ready" :items="data?.ready ?? []"/>
      <WorkItemTable title="In Progress" :items="data?.in_progress ?? []"/>
      <WorkItemTable title="Completed" :items="data?.completed ?? []"/>
    </div>
  </div>
</template>
