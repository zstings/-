<script setup>
  import { reactive } from 'vue';
  import { Command } from '@tauri-apps/api/shell';
  import { exists } from '@tauri-apps/api/fs';
  const form = reactive({
    originalPath: '1',
    targetPath: '1',
  });
  const handleSubmit = async (data) => {
    let {targetPath, originalPath} = data
    targetPath = targetPath.replaceAll('/', '\\')
    originalPath = originalPath.replaceAll('/', '\\')
    console.log(targetPath, originalPath, data);
    const isOriginalPath = await checkFileExists(originalPath);
    if (isOriginalPath) {
      // const output = await new Command('powershell', ['-Command', `Move-Item -Path ${originalPath} -Destination ${targetPath}`]).execute();
      // console.log(output)
      // 检查目标目录是否存在，不存在需要创建
      
    }

    // const command = new Command('powershell', ['-Command', 'New-Item -ItemType SymbolicLink -Path E:\\MyApp\\niva\\a -Target E:\\MyApp\\niva\\kan\\a']);
    // const command = new Command('powershell', ['-Command', 'Move-Item -Path E:\\MyApp\\niva\\kan\\a -Destination E:\\MyApp\\niva\\']);
    // command.stdout.on('data', data => {
    //   console.log(data);
    // });
    // command.stderr.on('data', data => {
    //   console.error(`stderr: ${data}`, data);
    // });
    // command.on('close', code => {
    //   console.log(`child process exited with code`, code);
    // });
    // command.spawn();
    // const child = await command.spawn();
    // console.log('pid:', child.pid);

  };
  const formItem = reactive([
    {
      field: 'originalPath',
      label: '原路径',
      uiAtter: {
        placeholder: '请输入原路径'
      },
      rules: [
        {required: true, message:'原路径必须填写'}
      ],
      validateTrigger: ['change', 'input'],
      tooltip: '文件或目录的默认路径，是被转移的路径'
    },{
      field: 'targetPath',
      label: '目标路径',
      rules: [
        {required: true, message:'目标路径必须填写'}
      ],
      validateTrigger: ['change', 'input'],
      uiAtter: {
        placeholder: '请输入目标路径'
      },
      tooltip: '文件或目录真实存放的路径，是需转移的路径'
    }
  ])
  async function checkFileExists (filePath) {
    return await exists(filePath);
  }
</script>

<template>
  <div class="container">
    <a-form :model="form" auto-label-width @submit-success="handleSubmit">
      <a-form-item
        v-for="(item, index) in formItem" :key="index"
        :field="item.field"
        :label="item.label"
        :rules="item.rules"
        :validate-trigger="item.validateTrigger"
        :tooltip="item.tooltip"
      >
        <a-input v-model="form[item.field]" v-bind="item.uiAtter" allow-clear/>
      </a-form-item>
      <a-form-item>
        <a-button html-type="submit" type="primary">开始迁移</a-button>
      </a-form-item>
    </a-form>
  </div>
</template>

<style>
.container {
  padding: 20px 20px 0;
}
.logo {
  &.vue {
    &:hover {
      filter: drop-shadow(0 0 2em #249b73);
    }
  }
  &.vite {
    &:hover {
      filter: drop-shadow(0 0 2em #747bff);
    }
  }
}
</style>
