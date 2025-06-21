# 社区资源板块代码开发指引

## 0. 前言

此文件适用于 [/src/views/download/Comps.vue](../src/views/download/Comps.vue) 的开发

## 1. 类型文件

类型文件应当存放于 [/src/types](../src/types) 中，其中

键值对键定义文件应当存放在 [/src/types/key](../src/types/key) 中，并在 [/src/types/SearchOptions.ts](../src/types/SearchOptions.ts) 中统一导出
每一种卡片类型（选项类型）创建一个文件，文件名格式为 `[yyy]Key.ts`，其中 `[yyy]` 为选项类型的英文名

文件内的类型以 `I[xxx][yyy]Key` 命名类型，其中 `[xxx]` 为资源类型的英文名，`[yyy]` 为选项类型的英文名（与文件名部分相同）
若无需细分资源类型（即所有资源类型的情况下都相同），则 type 中的 `[xxx]` 可以被省略

## 2. (渲染用) 选项文件

渲染用选项文件应当存放于 [/src/modules/comps/ui](../src/modules/comps/ui) 中，并在 [/src/modules/comps/index.ts](../src/modules/comps/index.ts) 中统一导出

文件名格式为 `[yyy].ts`，其中 `[yyy]` 为选项类型的英文名

文件内具名导出 `[xxx]_[yyy]_filters_inst`，其中 `[xxx]` 为资源类型的英文名，`[yyy]` 为选项类型的英文名，小写
导出对象类型为 `{ key: [key]; default?: boolean }[]`，其中 `[key]` 为对应类型（`I[xxx][yyy]Key`）

## 3. (绑定用) 选项文件

绑定用选项文件应当存放于 [/src/modules/comps/store](../src/modules/comps/store) 中，并在 [/src/modules/comps/index.ts](../src/modules/comps/index.ts) 中统一导出

文件名格式为 `[yyy].ts`，其中 `[yyy]` 为选项类型的英文名

文件内具名导出 `search_[xxx]_[yyy]_inst`，其中 `[xxx]` 为资源类型的英文名，`[yyy]` 为选项类型的英文名，小写
导出对象类型为 `Ref<Record<[key], boolean>>`，其中 `[key]` 为对应类型（`I[xxx][yyy]Key`）

## 4. 句柄

句柄文件应当存放于 [/src/modules/comps/handler](../src/modules/comps/handler) 中，并在 [/src/modules/comps/index.ts](../src/modules/comps/index.ts) 中统一导出

句柄文件内具名导出 `handle[xxx][yyy]Inst`，其中 `[xxx]` 为资源类型的英文名，`[yyy]` 为选项类型的英文名，首字母大写
句柄函数类型为 `(search_mod_loader: typeof [search_inst], type: "select" | "exclude", key: [key], value: boolean) => void`，其中
`type: "select" | "exclude"` 字段可选；`[search_inst]` 为绑定用选项（`search_[xxx]_[yyy]_inst`），`[key]` 为对应类型（`I[xxx][yyy]Key`）
