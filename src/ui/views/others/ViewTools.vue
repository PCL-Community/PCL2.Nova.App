<script setup lang="ts">
    import { ModEventBus } from "../../../modules/ModEventBus";
    import CompButton from "../../components/CompButton.vue";
    import CompCard from "../../components/CompCard.vue";
</script>

<template>
    <main>
        <CompCard title="？？？" can-swap is-swapped>
            <CompButton class="w-full" @click="ModEventBus.emit('router:push', '/firework')">
                <span>在主页上放烟花</span>
            </CompButton>
        </CompCard>
    </main>
</template>
