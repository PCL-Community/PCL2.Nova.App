<script setup lang="ts">
    import { ref, watch, watchEffect } from "vue";

    const props = defineProps<{
        state: "loading" | "error";
        message: string;
        error?: string;
    }>();

    const hammerRef = ref<SVGPathElement>();
    const trainglesRef = ref<SVGPathElement>();

    watchEffect(() => {
        const playState = props.state === "loading" ? "running" : "paused";
        hammerRef.value && (hammerRef.value.style.animationPlayState = playState);
        trainglesRef.value && (trainglesRef.value.style.animationPlayState = playState);
    });
</script>

<template>
    <div
        class="w-fit bg-[color-mix(in_oklch,_var(--color-base-100)_90%,_var(--color-base-content)_10%)] flex flex-col justify-center items-center gap-4 py-4 px-6 rounded-lg transition-all duration-150 ease-in-out"
        :class="{
            'text-info': $props.state === 'loading',
            'text-red-500': $props.state === 'error',
        }">
        <div>
            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="80" height="80" viewBox="0 0 80 80" fill="none" class="load-svg">
                <!-- 底下的横线 -->
                <path stroke="currentColor" stroke-width="2" stroke-linecap="round" d="M5 66L29 66"></path>
                <!-- 运动的锤子 -->
                <path
                    class="hammer"
                    ref="hammerRef"
                    d="M53.7693 59.6884L53.7763 27.5221C59.2904 27.6694 62.7075 30.1275 67.6568 31.1071C73.6227 32.4731 74.8377 33.7263 71.1429 29.9828C66.9914 25.9769 61.6417 23.1999 55.6855 22.2612C55.1971 20.2186 53.3582 18.7011 51.1671 18.7045C48.976 18.7078 47.138 20.2185 46.6487 22.2612C40.6892 23.2052 35.3415 25.9781 31.1913 29.9828C26.8344 34.236 28.2414 32.9395 34.6703 31.1142C39.6201 30.1326 43.0467 27.6687 48.5579 27.5221L48.5508 59.6884C48.5508 59.7233 48.5795 59.752 48.6145 59.752L53.7056 59.752C53.7434 59.7491 53.7663 59.7262 53.7693 59.6884Z"
                    stroke="currentColor"
                    stroke-width="2"></path>
                <!-- 锤子敲击后出现的三角形 -->
                <g class="triangles" fill="currentColor" ref="trainglesRef">
                    <path d="M19.7416 60.147L21.4566 62.0039L16.8852 64.5056"></path>
                    <path d="M11 61.67L12.8992 60.0019L15.2859 64.6345"></path>
                </g>
                <!-- 叉叉 -->
                <g class="wrong-x" fill="currentColor" v-show="$props.state === 'error'">
                    <path
                        d="M10.2929 49.2929C9.90237 49.6834 9.90237 50.3166 10.2929 50.7071L20.2929 60.7071C20.6834 61.0976 21.3166 61.0976 21.7071 60.7071C22.0976 60.3166 22.0976 59.6834 21.7071 59.2929L11.7071 49.2929C11.3166 48.9024 10.6834 48.9024 10.2929 49.2929Z"
                        fill-rule="evenodd"></path>
                    <path
                        d="M10.2929 60.7071C10.6834 61.0976 11.3166 61.0976 11.7071 60.7071L21.7071 50.7071C22.0977 50.3166 22.0976 49.6835 21.7071 49.2929C21.3166 48.9024 20.6834 48.9024 20.2929 49.2929L10.2929 59.2929C9.9024 59.6834 9.9024 60.3166 10.2929 60.7071Z"
                        fill-rule="evenodd"></path>
                </g>
            </svg>
        </div>
        <div v-if="state === 'loading'">{{ message }}</div>
        <div v-if="state === 'error'">
            {{ error }}
        </div>
    </div>
</template>

<style lang="css" scoped>
    .card-loading {
        width: 300px;
    }

    .loading-text {
        text-align: center;
        font-size: 14px;
    }

    @keyframes hammerAnim {
        0% {
            transform: rotate(10deg);
        }

        11% {
            transform: rotate(-55deg);
        }

        38% {
            transform: rotate(13deg);
        }

        46% {
            transform: rotate(4deg);
        }

        51% {
            transform: rotate(8deg);
        }

        73% {
            transform: rotate(6deg);
        }

        88% {
            transform: rotate(8deg);
        }

        100% {
            transform: rotate(10deg);
        }
    }

    @keyframes trianglesOpacity {
        0% {
            opacity: 0;
        }

        10% {
            opacity: 0;
        }

        13% {
            opacity: 1;
        }

        82% {
            opacity: 0;
        }

        100% {
            opacity: 0;
        }
    }

    .hammer {
        transform-origin: 50px 59px;
        animation: hammerAnim 1.2s infinite ease-in;
    }

    .load-svg > .triangles {
        animation: trianglesOpacity 1.2s infinite ease-in;
    }

    .load-svg > .wrong-x {
        opacity: 1;
        transition: opacity 0.4s ease;
    }

    .load-svg.loading > .wrong-x {
        opacity: 0;
    }

    #loading-container {
        flex-shrink: 0;
        display: flex;
        align-items: center;
        justify-content: center;
        /* background-color: antiquewhite; */
    }

    .load-svg {
        transition: color 0.4s ease;
    }
</style>
