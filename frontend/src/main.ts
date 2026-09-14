import { createApp } from 'vue'
import { createPinia } from 'pinia'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import * as ElIcons from '@element-plus/icons-vue'

import App from './App.vue'
import router from './router'
import { setupPermissionDirective } from './directives/permission'

const app = createApp(App)

// 注册全部 Element Plus 图标(学习项目简单粗暴,生产可按需)
for (const [name, comp] of Object.entries(ElIcons)) {
  app.component(name, comp as any)
}

app.use(createPinia())
app.use(router)
app.use(ElementPlus)
setupPermissionDirective(app)

app.mount('#app')
