import {ref} from "vue"

export const b_title = ref("")
export const b_content = ref("")
export const b_level = ref(0)
export const b_button = ref(['ok'])
export const b_show_all = ref(false)
export const b_resolve = ref<((value: number) => void) | null>(null)
export const b_show = ref(false)

export function messagebox(title: string, content: string, level: number = 0, button: string[] = ['ok']) {
    b_title.value = title
    b_content.value = content
    b_level.value = level
    b_button.value = button
    b_show.value = true
    b_show_all.value = false
    return new Promise((resolve) => {
        b_show_all.value = true
        b_resolve.value = resolve
    })
}