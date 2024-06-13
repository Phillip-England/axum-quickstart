



class App {
    constructor() {
        this.onMountFuncs = [];
    }
    onMount(func) {
        this.onMountFuncs.push(func);
    }
    mount() {
        this.onMountFuncs.forEach(func => func());
    }
}

const app = new App();