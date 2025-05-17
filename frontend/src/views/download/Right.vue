<script setup lang="ts">
import {ref, watch} from 'vue'
import {current_download} from '../../logic/changeBody'
import AutoInstall from './content/AutoInstall.vue'
import ManualInstall from './content/ManualInstall.vue'
import ExtensionMod from './content/ExtensionMod.vue'
import ExtensionModpack from './content/ExtensionModpack.vue'
import ExtensionResourcepack from './content/ExtensionResourcepack.vue'
import ExtensionShaderpack from './content/ExtensionShaderpack.vue'

const isTransitioning = ref(true)

function control_leave() {
  isTransitioning.value = true
}

watch(current_download, () => isTransitioning.value = false)
</script>
<template>
  <div>
    <transition name="slide" @after-leave="control_leave">
      <AutoInstall v-if="current_download == 'Auto-Install' && isTransitioning" class="component"
                   style="background-color: red;"/>
    </transition>
    <transition name="slide" @after-leave="control_leave">
      <ManualInstall v-if="current_download == 'Manual-Install' && isTransitioning" class="component"
                     style="background-color: blue;"/>
    </transition>
    <transition name="slide" @after-leave="control_leave">
      <ExtensionMod v-if="current_download == 'Extension-Mod' && isTransitioning" class="component"
                    style="background-color: green;"/>
    </transition>
    <transition name="slide" @after-leave="control_leave">
      <ExtensionModpack v-if="current_download == 'Extension-Modpack' && isTransitioning" class="component"
                        style="background-color: yellow;"/>
    </transition>
    <transition name="slide" @after-leave="control_leave">
      <ExtensionResourcepack v-if="current_download == 'Extension-Resourcepack' && isTransitioning" class="component"
                             style="background-color: orange;"/>
    </transition>
    <transition name="slide" @after-leave="control_leave">
      <ExtensionShaderpack v-if="current_download == 'Extension-Shaderpack' && isTransitioning" class="component"
                           style="background-color: purple;"/>
    </transition>
  </div>
</template>
<style scoped>
.component {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}

.slide-enter-active {
  animation: rightSlideIn 0.2s;
}

.slide-leave-active {
  animation: rightSlideOut 0.2s;
}

@keyframes rightSlideIn {
  from {
    transform: translateY(-100%);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

@keyframes rightSlideOut {
  from {
    transform: translateY(0);
    opacity: 1;
  }
  to {
    transform: translateY(-100%);
    opacity: 0;
  }
}
</style>
<script lang="ts">
export default {
  name: "downloadRight"
}
</script>