<script setup lang="ts">
    import { ref } from "vue";
    import CompCard from "../../components/CompCard.vue";
    import CompRadioButton from "../../components/CompRadioButton.vue";

    type TVersionType = "release" | "snapshot" | "old" | "april";
    type TVersion =
        | "1.21.x"
        | "1.20.x"
        | "1.19.x"
        | "1.18.x"
        | "1.17.x"
        | "1.16.x"
        | "1.15.x"
        | "1.14.x"
        | "1.13.x"
        | "1.12.x"
        | "1.11.x"
        | "1.10.x"
        | "1.9.x"
        | "1.8.x"
        | "1.7.x-"
        | "b"
        | "a"
        | "c"
        | "rd";

    const type = ref<TVersionType>("release");
    const version = ref<TVersion>("1.21.x");
    const refreshing = ref(false);

    const handleUpdate = (value: TVersionType | TVersion, index: 0 | 1) => {
        if (index === 0) {
            type.value = value as TVersionType;
            localStorage.setItem("X-Cache-Download-Auto-Type", type.value);
        } else {
            version.value = value as TVersion;
        }
    };
</script>

<template>
    <section>
        <header>
            <CompCard title="自动安装 | 快速选择器">
                <div class="px-6 flex flex-col gap-2" v-if="!refreshing">
                    <ul class="flex gap-2 items-center">
                        <span>版本类型：</span>
                        <CompRadioButton text="正式版" :checked="type === 'release'" highlight noicon @click="handleUpdate('release', 0)" />
                        <CompRadioButton
                            text="快照版"
                            :checked="type === 'snapshot'"
                            highlight
                            noicon
                            @click="handleUpdate('snapshot', 0)" />
                        <CompRadioButton text="远古版" :checked="type === 'old'" highlight noicon @click="handleUpdate('old', 0)" />
                        <CompRadioButton text="愚人节版" :checked="type === 'april'" highlight noicon @click="handleUpdate('april', 0)" />
                    </ul>
                    <ul class="flex gap-2 items-center" v-if="['release', 'snapshot'].includes(type)">
                        <span>版本：</span>
                        <CompRadioButton
                            text="1.21.x"
                            :checked="version === '1.21.x'"
                            highlight
                            noicon
                            @click="handleUpdate('1.21.x', 1)" />
                        <CompRadioButton
                            text="1.20.x"
                            :checked="version === '1.20.x'"
                            highlight
                            noicon
                            @click="handleUpdate('1.20.x', 1)" />
                        <CompRadioButton
                            text="1.19.x"
                            :checked="version === '1.19.x'"
                            highlight
                            noicon
                            @click="handleUpdate('1.19.x', 1)" />
                        <CompRadioButton
                            text="1.18.x"
                            :checked="version === '1.18.x'"
                            highlight
                            noicon
                            @click="handleUpdate('1.18.x', 1)" />
                        <CompRadioButton
                            text="1.17.x"
                            :checked="version === '1.17.x'"
                            highlight
                            noicon
                            @click="handleUpdate('1.17.x', 1)" />
                        <CompRadioButton
                            text="1.16.x"
                            :checked="version === '1.16.x'"
                            highlight
                            noicon
                            @click="handleUpdate('1.16.x', 1)" />
                        <CompRadioButton
                            text="1.15.x"
                            :checked="version === '1.15.x'"
                            highlight
                            noicon
                            @click="handleUpdate('1.15.x', 1)" />
                        <CompRadioButton
                            text="1.14.x"
                            :checked="version === '1.14.x'"
                            highlight
                            noicon
                            @click="handleUpdate('1.14.x', 1)" />
                    </ul>
                    <ul class="flex gap-2 items-center pl-14" v-if="['release', 'snapshot'].includes(type)">
                        <CompRadioButton
                            text="1.13.x"
                            :checked="version === '1.13.x'"
                            highlight
                            noicon
                            @click="handleUpdate('1.13.x', 1)" />
                        <CompRadioButton
                            text="1.12.x"
                            :checked="version === '1.12.x'"
                            highlight
                            noicon
                            @click="handleUpdate('1.12.x', 1)" />
                        <CompRadioButton
                            text="1.11.x"
                            :checked="version === '1.11.x'"
                            highlight
                            noicon
                            @click="handleUpdate('1.11.x', 1)" />
                        <CompRadioButton
                            text="1.10.x"
                            :checked="version === '1.10.x'"
                            highlight
                            noicon
                            @click="handleUpdate('1.10.x', 1)" />
                        <CompRadioButton text="1.9.x" :checked="version === '1.9.x'" highlight noicon @click="handleUpdate('1.9.x', 1)" />
                        <CompRadioButton text="1.8.x" :checked="version === '1.8.x'" highlight noicon @click="handleUpdate('1.8.x', 1)" />
                        <CompRadioButton
                            text="1.7.x 及以下"
                            :checked="version === '1.7.x-'"
                            highlight
                            noicon
                            @click="handleUpdate('1.7.x-', 1)" />
                    </ul>
                    <ul class="flex gap-2 items-center" v-if="type === 'old'">
                        <span>版本：</span>
                        <CompRadioButton text="b" :checked="version === 'b'" highlight noicon @click="handleUpdate('b', 1)" />
                        <CompRadioButton text="a" :checked="version === 'a'" highlight noicon @click="handleUpdate('a', 1)" />
                        <CompRadioButton text="c" :checked="version === 'c'" highlight noicon @click="handleUpdate('c', 1)" />
                        <CompRadioButton text="rd" :checked="version === 'rd'" highlight noicon @click="handleUpdate('rd', 1)" />
                    </ul>
                </div>
            </CompCard>
        </header>
        <main></main>
    </section>
</template>
