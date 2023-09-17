<script lang="ts" setup>
import MarkdownIt from "markdown-it";
const markdown = new MarkdownIt()

defineProps({
    text: {
        type: String,
        default: ""
    }
});
</script>

<template>
    <div v-html="markdown.render(text)" />
</template>

<style scoped></style>