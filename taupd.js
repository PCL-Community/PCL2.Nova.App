import fs from "fs";
import inquirer from "inquirer";
import chalk from "chalk";

function versionCheck(version) {
    if (!version) return "版本号不能为空";
    if ((version.match(/\./g) || []).length !== 2) return "版本号格式错误";
    if (!/^\d{1,2}\.\d{1,2}\.\d{1,2}$/.test(version)) return "版本号格式错误";
    return true;
}

function check(data) {
    if (!data) {
        console.log(chalk.ansi256(160)("[F] 信息不完整，更新失败"));
        process.exit(1);
    }
}

console.log(chalk.ansi256(214)("\n[I] Nova 项目版本号更新工具 v1.0.0\n"));

(async function main() {
    // const { method } = await inquirer.prompt({
    //     type: "list",
    //     name: "method",
    //     message: "请选择更新模式",
    //     choices: ["Tauri v2 CI with Metadata in src folder"],
    // });
    // check(method);

    const { channel } = await inquirer.prompt({
        type: "list",
        name: "channel",
        message: "请选择更新渠道",
        choices: ["Dev", "Beta", "Stable"],
    });
    check(channel);

    const { version } = await inquirer.prompt({
        type: "input",
        name: "version",
        message: "请输入版本号",
        validate: versionCheck,
    });
    check(version);

    chalk.reset();

    // if (method === "Tauri v2 CI with Metadata in src folder") {
        console.log(chalk.ansi256(129)("\n[I] 开始以 Tauri v2 CI (Metadata in src folder) 模式更新\n"));

        // 更新 package.json
        try {
            let packageJson = JSON.parse(fs.readFileSync("package.json", "utf8"));
            packageJson.version = version;
            fs.writeFileSync("package.json", JSON.stringify(packageJson, null, 4));
            console.log(chalk.ansi256(40)("[I] 更新 ./package.json 文件成功"));
        } catch (error) {
            console.log(chalk.ansi256(220)("[W] 更新 ./package.json 文件失败：\n      " + error.message + "\n    已跳过更新"));
        }

        // 更新 tauri.conf.json
        try {
            let conf = JSON.parse(fs.readFileSync("./src-tauri/tauri.conf.json", "utf8"));
            conf.version = version;
            conf.productName = `Plain Craft Launcher 2 Nova ${channel}`;
            fs.writeFileSync("./src-tauri/tauri.conf.json", JSON.stringify(conf, null, 4));
            console.log(chalk.ansi256(40)("[I] 更新 ./src-tauri/tauri.conf.json 文件成功"));
        } catch (error) {
            console.log(chalk.ansi256(220)("[W] 更新 ./src-tauri/tauri.conf.json 文件失败：\n      " + error.message + "\n    已跳过更新"));
        }

        // 更新 Cargo.toml
        try {
            let cargo = fs.readFileSync("./src-tauri/Cargo.toml", "utf8");
            cargo = cargo.replace(/version = \"\d{1,2}\.\d{1,2}\.\d{1,2}\"/, `version = "${version}"`);
            fs.writeFileSync("./src-tauri/Cargo.toml", cargo);
            console.log(chalk.ansi256(40)("[I] 更新 ./src-tauri/Cargo.toml 文件成功"));
        } catch (error) {
            console.log(chalk.ansi256(220)("[W] 更新 ./src-tauri/Cargo.toml 文件失败：\n      " + error.message + "\n    已跳过更新"));
        }

        // 更新 metadata.json
        try {
            let metadata = JSON.parse(fs.readFileSync("./src/metadata.json", "utf8"));
            metadata.channel = channel;
            metadata.version = version;
            fs.writeFileSync("./src/metadata.json", JSON.stringify(metadata, null, 4));
            console.log(chalk.ansi256(40)("[I] 更新 ./src/metadata.json 文件成功"));
        } catch (error) {
            console.log(chalk.ansi256(220)("[W] 更新 ./src/metadata.json 文件失败：\n      " + error.message + "\n    已跳过更新"));
        }

        console.log(chalk.ansi256(40)("\n[I] 更新完成"));
    // }

    chalk.reset();
    process.exit(0);
})();
