<script setup lang="ts">
import type {WorkItem} from "../interfaces/workItem";

defineProps<{
  title: string,
  items: WorkItem[]
}>();

</script>

<template>
  <div>
    <h2>{{ title }}</h2>
    <table>
      <thead>
      <tr>
        <th>Title</th>
        <th>Size</th>
        <th>Status</th>
      </tr>
      </thead>
      <tbody>
      <tr v-for="(item,index) in items" :key="index">
        <td>{{ item.title }}</td>
        <td>{{ item.size }}</td>
        <td>{{ item.status }}</td>
      </tr>
      </tbody>
    </table>
  </div>
</template>