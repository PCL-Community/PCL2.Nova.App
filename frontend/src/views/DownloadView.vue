<template>
    <RouterView v-slot="{ Component }">
        <section class="w-full h-[calc(100vh-calc(var(--spacing)*12))] p-4" :key="$route.fullPath">
            <component :is="Component" />
        </section>
    </RouterView>
</template>
