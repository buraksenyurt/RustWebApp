<script setup lang="ts">
import {ref, onMounted} from "vue";
import getAll from "./api/get";
import type {WorkItems} from './interfaces/workItems';
import WorkItemTable from "./components/WorkItemTable.vue";

const data = ref<WorkItems | null>(null);
const loading = ref(true);
const error = ref<string | null>(null);

onMounted(async () => {
  const result = await getAll();
  if ('data' in result) {
    data.value = result.data;
  } else {
    error.value = result.error ?? 'Unknown error';
  }
  loading.value = false;
});
</script>

<template>
  <div>
    <h2>Work Item Dashboard</h2>
    <p v-if="loading">Loading</p>
    <p v-else-if="error">Error: {{ error }}</p>
    <div v-else>
      <WorkItemTable title="Ready" :items="data?.ready ?? []"/>
      <WorkItemTable title="In Progress" :items="data?.inProgress ?? []"/>
      <WorkItemTable title="Completed" :items="data?.completed ?? []"/>
    </div>
  </div>
</template>
