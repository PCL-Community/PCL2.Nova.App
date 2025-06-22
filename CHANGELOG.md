# 0.1.0-beta.1

refactor(ui): ui-next with daisyui [step 1]

```diff
+ add tailwindcss
+ add daisyui
+ add iconify & iconify-json
! re-design logo
! re-design ui
! improve code strucute
! improve CHANGELOG format
- remove old files
- remove herobrine
```

# 0.0.12

1. 修改了 MyLoading 的样式，现在成功和失败时的左下角都会有一个图标了。
2. 修正了一下 MySelectCard 组件，现在已经不用 maxHeight 了~ （[PR#27](https://github.com/PCL-Community/PCL2.Nova.App/pull/27)，[@AMagicPear](https://github.com/AMagicPear)）
3. 新组件：MyCheckButton！（同时修改原来的 MyCheckButton 名称为 MyNavButton）【与 MyNavButton 不同的是，该 button 选中时中间会打勾~】
    1. 👆 感觉完全可以被 MyToggleSwitch 代替（但是又转念一想，好像也可以（目前 MyCheckButton 仅在主界面有展示，在别的地方目前没有实用空间。。
4. 初步设置透明度，并且现在已经支持设置背景图片啦！只需要将背景图片放到`{exe}\PCL.Nova\BackgroundImage`文件夹下，即可随机从中抽取一张背景图片展示！！
5. 现在开始，所有**组件**均有透明度 127 了！不用担心遮挡住背景图片了！
6. 稍微调亮了一点加载框成功时的颜色（
7. 在关于与鸣谢部分新增了 PCL Community!

# 0.0.11

1. 翻新账号部分！现在终于有 CE 那味了~~（虽然现在还暂时无法登录微软，不过可以登录离线登录啦！！）
2. 调整了一下信息框的背景颜色，现在看起来应该会更柔和了一点吧~
3. 稍微调整了一下加载框的暗色模式颜色。现在中间的镐子动画应该更加明显了。
4. 为 NormalButton 新增了 disabled 属性！现在可以正常显示禁用的样式啦~（不过正常的话需要手动判断此时是否处在禁用状态。。）
5. 账号部分目前可以正常保存到外部文件了！【别轻易发给别人（】
6. 为 Nova 添加了 exe 的图标啦！

# 0.0.10

1. 更了个新的控件：MyRadioButton！
2. 现在标题栏在从亮色切换到暗色时，亮度会稍微变暗一点。
3. 现在，暗色模式会保存到外部文件了！第一次启动时，会生成`{exe}\PCL.Nova\config\PCL2.Nova.ini`文件，可以保存一些配置信息。
4. 新增关于与鸣谢啦~【添加了原作者、现作者的鸣谢人员等】

# 0.0.9

1. 修复了扫雷无论点不点击扣分模式，都会扣分的 bug
2. 将外置登录提上日程！！

# 0.0.8

<<<<<<< HEAD
1. MyToggleSwitch 控件应用暗色模式！
2. 扫雷游戏新增了【扣分模式】，此时扫雷如果踩中雷不会扣分，而是减 50 分。
3. 2048 游戏新增【作弊生成】按钮，按下即可生成 256、512、1024 三种随机数字在场上。ps：该操作不扣分也不加分，不减步数也不加步数。

# 0.0.7

1. 在主界面的 MyPrograssBar 底下的文字新增了【进度显示】
2. 更新了 MyPrograssBar 的深色模式！
3. 更新了 2048 小游戏！可以去玩了！

# 0.0.6

1. 更新了 MyPrograssBar 控件！
2. 在主界面新增了【测试进度条】框，现在可以点击【开始】，然后进行测试加载了！

# 0.0.5

1. 更换架构至`wails`，由于相较之前并没有什么新增内容，因此暂不发布二进制文件。。

# 0.0.4

1. 更新了 MyToggleSwitch（自定义拨动开关）【应用至暗色模式的调整】
2. 更新了扫雷游戏！（更多 -> 小游戏 -> 扫雷），完美仿刻于：[xphost 的 Godot 扫雷游戏](https://github.com/xphost008/Godot-Minesweeper)
3. 目前该扫雷缺陷是：仅仿刻于 0.0.3 版本，并且没有种子功能，同时游戏开局可能会直接踩雷直接输（因为开局就已经确定了所有雷的位置。。

# 0.0.3

1. 首次公开发布，用时两周
2. 目前更新自定义控件：MyDialog（自定义信息框）、MyLoading（自定义加载框）、MySelectCard（自定义下拉卡片）等一系列自定义组件
3. 更新了黑暗模式
=======
1. 修改了MyLoading的样式，现在成功和失败时的左下角都会有一个图标了。
2. 修正了一下MySelectCard组件，现在已经不用maxHeight了~ （[PR#27](https://github.com/PCL-Community/PCL2.Nova.App/pull/27)，[@AMagicPear](https://github.com/AMagicPear)）
3. 新组件：MyCheckButton！（同时修改原来的MyCheckButton名称为MyNavButton）【与MyNavButton不同的是，该button选中时中间会打勾~】
   1. 👆感觉完全可以被MyToggleSwitch代替（但是又转念一想，好像也可以（目前MyCheckButton仅在主界面有展示，在别的地方目前没有实用空间。。
4. 初步设置透明度，并且现在已经支持设置背景图片啦！只需要将背景图片放到`{exe}\PCL.Nova\BackgroundImage`文件夹下，即可随机从中抽取一张背景图片展示！！
5. 现在开始，所有**组件**均有透明度127了！不用担心遮挡住背景图片了！
6. 稍微调亮了一点加载框成功时的颜色（
7. 在关于与鸣谢部分新增了PCL Community!
8. 将鸣谢改成特别鸣谢~
