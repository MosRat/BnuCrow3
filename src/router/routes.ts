import type {RouteRecordRaw} from 'vue-router'


const routes: Array<RouteRecordRaw> = [
    {
        path: '/login',
        name: 'login',
        alias: '/',
        meta: {
            hideComponents: true,
            bg:'linear-gradient(120deg, #e0c3fc 0%, #8ec5fc 100%)'
        },
        component: () => import('@view/Login.vue'),
    },
    {
        path: '/actlist',
        name: 'actlist',
        // meta:{
        //     bg:'linear-gradient(to top, #e6e9f0 0%, #eef1f5 100%)'
        // },
        component: () => import('@view/ActionList.vue'),
    },
    {
        path: '/exam',
        name: 'exam',
        // meta:{
        //     bg:'linear-gradient(to top, #e6e9f0 0%, #eef1f5 100%)'
        // },
        component: () => import('@view/Exam.vue'),
    },
    {
        path: '/score',
        name: 'score',
        // meta:{
        //     bg:'linear-gradient(to top, #e6e9f0 0%, #eef1f5 100%)'
        // },
        component: () => import('@view/Score.vue'),
    },
    {
        path: '/tauri',
        name: 'tauri',
        component: () => import('@view/Tauri.vue'),
    },
    {
        path: '/classtable',
        name: 'classtable',
        meta: {
            // bg: 'linear-gradient(to top, #fbc2eb 0%, #a6c1ee 100%)'
            bg: 'linear-gradient(to top, #d5d4d0 0%, #d5d4d0 1%, #eeeeec 31%, #efeeec 75%, #e9e9e7 100%)'
        },
        component: () => import('@view/ClassTable.vue'),
    },
    {
        path: '/:pathMatch(.*)*\'',
        name: 'Not Found',
        component: () => import('@view/NotFound.vue'),
    },
]


export default routes