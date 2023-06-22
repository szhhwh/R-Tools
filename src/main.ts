import { createApp } from "vue"
import "./styles.css"
import App from "./App.vue"
import { useRouter,useRoute } from "vue-router"

const Home = { template: '<div>Home</div>' }
const About = { template: '<div>About</div>' }

const router = useRouter()
const route = useRoute()

function pushWithQuery(query: any) {
    router.push({
      name: 'search',
      query: {
        ...route.query,
        ...query,
      },
    })
  }


const app = createApp({})

app.use(router)

app.mount('#app')
