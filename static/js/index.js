
async function sleep(ms) {
    await new Promise(resolve => setTimeout(resolve, ms));
}

function qs(selector) {
    return document.querySelector(selector);
}

function qsa(selector) {
    return document.querySelectorAll(selector);
}

function onClick(element, callback) {
    element.addEventListener('click', async (e) => {
        await callback(e);
    });
}


class Aktr {
    constructor() {
        this.onMountFuncs = [];
    }
    onMount(func) {
        this.onMountFuncs.push(func);
    }
    async mount() {
        this.onMountFuncs.forEach(async func => await func());
        this.onMountFuncs = [];
    }
}

const aktr = new Aktr();