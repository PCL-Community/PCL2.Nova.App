<script setup lang="ts">
    const props = defineProps<{
        uuid?: string;
        name?: string;
        defaultSkin?: "Steve" | "Alex";
    }>();
</script>

<template>
    <div class="SkinAvatar relative w-16 h-16 rounded-sm overflow-hidden shadow-[0_0_8px_2px] shadow-primary-content/30">
        <img v-if="!props.defaultSkin && (props.uuid || props.name)" :src="`https://minotar.net/helm/${props.uuid || props.name}`" />
        <img
            v-if="props.defaultSkin === 'Alex'"
            src="data:image/png;base64, iVBORw0KGgoAAAANSUhEUgAAALQAAAC0CAIAAACyr5FlAAACzklEQVR4nOzcMWplZRiA4XM9R28CgqVlWu3Eyk679C7Bxsrawm0IksZluARL7dTKzsJmwgwzTBISZsgC3jTfgbkMz7OAj+/Ay1/959/+u7pchl7eTCcsy/Lx2UmsMTf/kF2+ZY81PpiP4H0lDpI4SOIgiYMkDpI4SOIgiYMkDpI4SOIgiYMkDpI4SOIgiYMkDpI4SOIgiYO0nchF5xNZ4/Xdq+GE8/kSJ3MX38lBEgdJHCRxkMRBEgdJHCRxkMRBEgdJHCRxkMRBEgdJHCRxkMRBEgdJHCRxkMRBEgdpO5Fb8PM15n8VLMvyyTc/zofM3f3+87teYXFy8BRxkMRBEgdJHCRxkMRBEgdJHCRxkMRBEgdJHCRxkMRBEgdJHCRxkMRBEgdJHKRth0vbH+2xyHiNXS6OH4/H4YQXz6/nayxffjcccPjj1/kWTg6SOEjiIImDJA6SOEjiIImDJA6SOEjiIImDJA6SOEjiIImDJA6SOEjiIImDJA6SOEjb2Vc/DEd8+sW38z0++/7z4YQ/L6d/FTxap+/zf/3LT/Mt/rn6ezjh//kSTg6eIA6SOEjiIImDJA6SOEjiIImDJA6SOEjiIImDJA6SOEjiIImDJA6SOEjiIImDdHj212/TEeuH8z3Ww5vxiOnF8UcPN9MJh3W+xf39/XDCth7mazg5SOIgiYMkDpI4SOIgiYMkDpI4SOIgiYMkDpI4SOIgiYMkDpI4SOIgiYMkDpI4SOIgbdu2TWese7xIP/4n4OHf6x3WGFsvzudD5j8W3N7eztdwcpDEQRIHSRwkcZDEQRIHSRwkcZDEQRIHSRwkcZDEQRIHSRwkcZDEQRIHSRwkcZDGV893eUx+j2ft14sdttjBLu/zjx33+CPAyUESB0kcJHGQxEESB0kcJHGQxEESB0kcJHGQxEESB0kcJHGQxEESB0kcJHGQxEESB+ltAAAA//8uGT8jBgUhDwAAAABJRU5ErkJggg==" />
        <img
            src="data:image/png;base64, iVBORw0KGgoAAAANSUhEUgAAALQAAAC0CAIAAACyr5FlAAAC4klEQVR4nOzcLWu9dRzH8aNc23E6nQbFYtNgMXgDJp3RYBJBrBa74IMQ7IrdMgwmwSQrYhFZETSaBefub0DxAbwt38kOf16vB/DlM3hzpd/O8srzT602wO72ct8TNsjp9e3wwuX4wmq1enh+ggeVOEjiIImDJA6SOEjiIImDJA6SOEjiIImDJA6SOEjiIImDJA6SOEjiIImDJA7Scnx6MTyxt7sz3zF/b70h79fnf8jm/C2+HCRxkMRBEgdJHCRxkMRBEgdJHCRxkMRBEgdJHCRxkMRBEgdJHCRxkMRBEgdJHCRxkJb19tbwxEdvvj7f8ewzTw8vLI8+Pp9xe34yvHBxdTOfcXz85/DCF4c/zmf4cpDEQRIHSRwkcZDEQRIHSRwkcZDEQRIHSRwkcZDEQRIHSRwkcZDEQRIHSRwkcZAe+vLDt4cn9vaenO/YWU8fwd+JO3k7vgnm79d9Ofgv4iCJgyQOkjhI4iCJgyQOkjhI4iCJgyQOkjhI4iCJgyQOkjhI4iCJgyQOkjhI4iAt838seO+zr+Y73n3j0+GF91/7eT5j/qL/21/35zO+PvxkeOHg4w/mM3w5SOIgiYMkDpI4SOIgiYMkDpI4SOIgiYMkDpI4SOIgiYMkDpI4SOIgiYMkDpI4SJvyw/hz3//0y31P+Nf+yy/e94SVH8bnfycOkjhI4iCJgyQOkjhI4iCJgyQOkjhI4iCJgyQOkjhI4iCJgyQOkjhI4iCJgyQO0nJ2fj48cXpzNt/x3Q9HwwsvPffCfMbc5wffzI+889arwwt/nd3MZ/hykMRBEgdJHCRxkMRBEgdJHCRxkMRBEgdJHCRxkMRBEgdJHCRxkMRBEgdJHCRxkJZl/ffwxB8nd/DQ+eryenjh6PffNmHG+pHt+Yz52/EnHtuaz/DlIImDJA6SOEjiIImDJA6SOEjiIImDJA6SOEjiIImDJA6SOEjiIImDJA6SOEjiIImD9E8AAAD//+BgVRpGjg3/AAAAAElFTkSuQmCC" />
    </div>
</template>

<style lang="scss" scoped>
    div.SkinAvatar {
        img {
            position: absolute;
            top: 0%;
            left: 0%;
            width: 100%;
            height: 100%;
        }
    }
</style>
