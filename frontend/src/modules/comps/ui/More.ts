import type { IFilterItem, IFilterItemNoIcon, IPerfKey, IResolutionsKey } from "@/types/SearchOptions";
import * as PerfIcon from "@/icons/perf";

export const Resolution: IFilterItemNoIcon<IResolutionsKey>[] = [
    { key: "8x-", name: "8x 或更低", default: true },
    { key: "16x", name: "16x", default: true },
    { key: "32x", name: "32x", default: true },
    { key: "48x", name: "48x", default: true },
    { key: "64x", name: "64x", default: true },
    { key: "128x", name: "128x", default: true },
    { key: "256x", name: "256x", default: true },
    { key: "512x+", name: "512x 或更高", default: true },
];

export const Perf: IFilterItem<IPerfKey>[] = [
    { key: "high", name: "高", icon: PerfIcon.High, default: true },
    { key: "medium", name: "中", icon: PerfIcon.Medium, default: true },
    { key: "low", name: "低", icon: PerfIcon.Low, default: true },
    { key: "potato", name: "土豆", icon: PerfIcon.Potato, default: true },
    { key: "screenshot", name: "截图", icon: PerfIcon.Screenshot, default: true },
];
