<script setup>
  import { reactive, computed } from 'vue';
  import { Message } from '@arco-design/web-vue';
  import { Command } from '@tauri-apps/api/shell';
  import { exists } from '@tauri-apps/api/fs';
  import { invoke } from "@tauri-apps/api/tauri";
  const form = reactive({
    // originalPath: 'F:\\Desktop\\exe\\ccc\\a',
    // targetPath: 'E:\\gr-xm\\kan\\a',
    originalPath: '',
    targetPath: '',
    loading: false
  });
  const handleSubmit = async (data) => {
    form.loading = true
    let { targetPath, originalPath } = data
    targetPath = targetPath.replaceAll('/', '\\')
    originalPath = originalPath.replaceAll('/', '\\')
    const tp1 = targetPath.split('\\').at(-1)
    const tp2 = targetPath.replace('\\' + tp1, '')
    const op1 = originalPath.split('\\').at(-1)
    const op2 = originalPath.replace('\\' + op1, '')
    // console.log(tp1, tp2, op1, op2);
    if (tp1 != op1) {
      Message.error('符号路径与实际路径最后一级文件或文件夹名必须一致')
      form.loading = false
      return
    }
    try {
      const isOriginalPath = await checkFileExists(originalPath);
      // console.log(isOriginalPath)
      if (isOriginalPath) {
        const isSymlink = await invoke("is_symlink", { path: originalPath });
        if (isSymlink) {
          await new Command('powershell', ['-Command', `rm -r -Force '${originalPath}'`]).execute();
        } else {
          if (await checkFileExists(targetPath)) {
            await new Command('powershell', ['-Command', `rm -r -Force '${targetPath}'`]).execute();
          }
          const move = await new Command('powershell', ['-Command', `Move-Item -Path '${originalPath}' -Destination '${tp2}'`]).execute();
          console.log(move.code == 0)
        }
      }
      await new Command('powershell', ['-Command', `New-Item -ItemType SymbolicLink -Path '${originalPath}' -Target '${targetPath}'`]).execute();
      Message.success('迁移成功')
      form.loading = false
      form.originalPath = ''
      form.targetPath = ''
    } catch(err) {
      console.log(err)
      Message.error(err)
      form.loading = false
    }

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
      label: '符号路径',
      uiAtter: {
        placeholder: '请输入符号路径: 文件或目录的默认路径，是被转移的路径'
      },
      rules: [
        {required: true, message:'符号路径必须填写'}
      ],
      validateTrigger: ['change', 'input'],
      tooltip: '文件或目录的默认路径，是被转移的路径'
    },{
      field: 'targetPath',
      label: '实际路径',
      rules: [
        {required: true, message:'实际路径必须填写'}
      ],
      validateTrigger: ['change', 'input'],
      uiAtter: {
        placeholder: '请输入实际路径 :文件或目录真实存放的路径，是需转移的路径'
      },
      tooltip: '文件或目录真实存放的路径，是需转移的路径'
    }
  ])
  async function checkFileExists (filePath) {
    return await exists(filePath);
  }
  const msCont = computed(() => {
    return form.targetPath.replaceAll('/', '\\').split('\\').at(-1)
  })
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
        >
        <!-- :tooltip="item.tooltip" -->
        <a-input v-model="form[item.field]" v-bind="item.uiAtter" allow-clear/>
      </a-form-item>
      <a-form-item>
        迁移内容为： {{ msCont }}
      </a-form-item>
      <a-form-item>
        <a-button html-type="submit" type="primary" :loading="form.loading">开始迁移</a-button>
      </a-form-item>
    </a-form>
  </div>
</template>

<style>
.container {
  padding: 40px 20px 0;
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
