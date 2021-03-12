import Groups from './views/Groups.vue'
import History from './views/History.vue'
import Contents from './views/Contents.vue'
import Dashboard from './views/Dashboard.vue'
import Download from './views/Download.vue'

const routes = [
    { path: '/', component: Dashboard },
    { path: '/group', component: Groups },
    { path: '/history', component: History },
    { path: '/contents', component: Contents },
    { path: '/download', component: Download },
];

export default routes;