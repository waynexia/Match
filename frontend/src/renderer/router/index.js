import Vue from 'vue'
import Router from 'vue-router'

Vue.use(Router)

export default new Router({
    routes: [
    {
        path: '/',
        redirect:'/frame'
    },
    {
        path: '/frame',
        component:resolve => require(['../frame.vue'],resolve),
        meta:
        {
            title:'导航栏'
        },
        props:true,
        redirect:'/home',
        children:
        [
        {
            path:'/home',
            name:'home',
            component:resolve =>require(['../components/page.vue'],resolve),
            meta:
            {
                title:'主页'
            },
            children:
            [

            ]
        }
        ]
    }]
})
