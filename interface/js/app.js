// ============================================================
// VirtualPC - APP.JS COMPLETO
// Criação de VM + gerenciamento + VM + mouse + teclado + API
// ============================================================

const VirtualPC = {

    version: "0.1.0",

    // ========================================================
    // VM STORAGE
    // ========================================================

    getVMs() {
        return JSON.parse(
            localStorage.getItem("virtualPC_vms") || "[]"
        );
    },

    saveVMs(vms) {
        localStorage.setItem(
            "virtualPC_vms",
            JSON.stringify(vms)
        );
    },

    getCurrentVM() {
        const id = Number(
            localStorage.getItem("virtualPC_current_vm")
        );

        return this.getVMs().find(
            vm => vm.id === id
        );
    },

    // ========================================================
    // CRIAR VM
    // ========================================================

    createVM() {

        const nome =
            document.getElementById("vmNome")?.value.trim();

        const cpu =
            Number(
                document.getElementById("vmCPU")?.value
            );

        const ram =
            Number(
                document.getElementById("vmRAM")?.value
            );

        const storage =
            Number(
                document.getElementById("vmStorage")?.value
            );

        if (!nome) {
            alert("Digite o nome da VM.");
            return;
        }

        if (!cpu || !ram || !storage) {
            alert("Preencha CPU, RAM e armazenamento.");
            return;
        }

        const vm = {
            id: Date.now(),

            nome: nome,

            cpu: cpu,

            ram: ram,

            storage: storage,

            status: "Desligada",

            criadaEm:
                new Date().toISOString()
        };

        const vms = this.getVMs();

        vms.push(vm);

        this.saveVMs(vms);

        localStorage.setItem(
            "virtualPC_current_vm",
            String(vm.id)
        );

        window.location.href =
            "minhas-vms.html";
    },

    // ========================================================
    // ABRIR VM
    // ========================================================

    openVM(id) {

        localStorage.setItem(
            "virtualPC_current_vm",
            String(id)
        );

        window.location.href =
            "vm.html";
    },

    // ========================================================
    // MOSTRAR VMS
    // ========================================================

    renderVMs() {

        const container =
            document.getElementById(
                "vmContainer"
            );

        if (!container) return;

        const vms = this.getVMs();

        if (vms.length === 0) {

            container.innerHTML = `
                <div class="empty-vms">

                    <div class="empty-vms-icon">
                        🖥️
                    </div>

                    <h2>Nenhuma VM criada</h2>

                    <p>
                        Crie sua primeira máquina virtual.
                    </p>

                </div>
            `;

            return;
        }

        container.innerHTML = "";

        vms.forEach(vm => {

            const card =
                document.createElement("div");

            card.className =
                "vm-card";

            card.innerHTML = `

                <div class="vm-card-top">

                    <div class="vm-icon">
                        🖥️
                    </div>

                    <div>

                        <h2>
                            ${vm.nome}
                        </h2>

                        <span class="vm-status">
                            ${vm.status}
                        </span>

                    </div>

                </div>

                <div class="vm-specs">

                    <div class="vm-spec">

                        <span>CPU</span>

                        <strong>
                            ${vm.cpu} núcleos
                        </strong>

                    </div>

                    <div class="vm-spec">

                        <span>RAM</span>

                        <strong>
                            ${vm.ram} GB
                        </strong>

                    </div>

                    <div class="vm-spec">

                        <span>DISCO</span>

                        <strong>
                            ${vm.storage} GB
                        </strong>

                    </div>

                </div>

                <button
                    class="open-vm-button"
                    onclick="VirtualPC.openVM(${vm.id})">

                    Abrir VM

                </button>
            `;

            container.appendChild(card);
        });
    },

    // ========================================================
    // CARREGAR VM ATUAL
    // ========================================================

    loadCurrentVM() {

        const vm =
            this.getCurrentVM();

        if (!vm) {

            if (
                location.pathname.endsWith(
                    "vm.html"
                )
            ) {
                window.location.href =
                    "minhas-vms.html";
            }

            return;
        }

        const nome =
            document.getElementById(
                "vmName"
            );

        const cpu =
            document.getElementById(
                "vmCpu"
            );

        const ram =
            document.getElementById(
                "vmRam"
            );

        const storage =
            document.getElementById(
                "vmStorage"
            );

        const status =
            document.getElementById(
                "vmStatus"
            );

        if (nome)
            nome.textContent =
                vm.nome;

        if (cpu)
            cpu.textContent =
                `${vm.cpu} núcleos`;

        if (ram)
            ram.textContent =
                `${vm.ram} GB`;

        if (storage)
            storage.textContent =
                `${vm.storage} GB`;

        if (status)
            status.textContent =
                vm.status;
    },

    // ========================================================
    // ATUALIZAR VM
    // ========================================================

    updateVM(vm) {

        const vms =
            this.getVMs();

        const index =
            vms.findIndex(
                item => item.id === vm.id
            );

        if (index === -1)
            return;

        vms[index] = vm;

        this.saveVMs(vms);
    },

    // ========================================================
    // INICIAR VM
    // ========================================================

    startVM() {

        const vm =
            this.getCurrentVM();

        if (!vm)
            return;

        vm.status =
            "Ligada";

        this.updateVM(vm);

        this.loadCurrentVM();
    },

    // ========================================================
    // DESLIGAR VM
    // ========================================================

    stopVM() {

        const vm =
            this.getCurrentVM();

        if (!vm)
            return;

        vm.status =
            "Desligada";

        this.updateVM(vm);

        this.loadCurrentVM();
    },

    // ========================================================
    // EXCLUIR VM ATUAL
    // ========================================================

    deleteCurrentVM() {

        const vm =
            this.getCurrentVM();

        if (!vm)
            return;

        if (
            !confirm(
                `Apagar a VM "${vm.nome}"?`
            )
        )
            return;

        const vms =
            this.getVMs().filter(
                item =>
                    item.id !== vm.id
            );

        this.saveVMs(vms);

        localStorage.removeItem(
            "virtualPC_current_vm"
        );

        window.location.href =
            "minhas-vms.html";
    },

    // ========================================================
    // EXCLUIR TODAS AS VMS
    // ========================================================

    deleteAllVMs() {

        if (
            !confirm(
                "Deseja apagar todas as VMs?"
            )
        )
            return;

        localStorage.removeItem(
            "virtualPC_vms"
        );

        localStorage.removeItem(
            "virtualPC_current_vm"
        );

        window.location.href =
            "minhas-vms.html";
    },

    // ========================================================
    // NAVEGAÇÃO
    // ========================================================

    openSettings() {

        window.location.href =
            "configuracoes.html";
    },

    backToVMs() {

        window.location.href =
            "minhas-vms.html";
    },

    // ========================================================
    // MOUSE VIRTUAL
    // ========================================================

    mouse: {

        x: 50,

        y: 50,

        sensitivity: 0.15,

        move(dx, dy) {

            this.x +=
                dx * this.sensitivity;

            this.y +=
                dy * this.sensitivity;

            this.x =
                Math.max(
                    0,
                    Math.min(
                        100,
                        this.x
                    )
                );

            this.y =
                Math.max(
                    0,
                    Math.min(
                        100,
                        this.y
                    )
                );

            this.update();
        },

        update() {

            const cursor =
                document.querySelector(
                    ".virtual-cursor, .vm-cursor"
                );

            if (!cursor)
                return;

            cursor.style.left =
                `${this.x}%`;

            cursor.style.top =
                `${this.y}%`;
        },

        leftClick() {

            console.log(
                "Mouse: clique esquerdo"
            );

            VirtualPC.API.sendMouse({
                type: "left_click",
                x: this.x,
                y: this.y
            });
        },

        rightClick() {

            console.log(
                "Mouse: clique direito"
            );

            VirtualPC.API.sendMouse({
                type: "right_click",
                x: this.x,
                y: this.y
            });
        }
    },

    // ========================================================
    // TOUCH / MOUSE ESTILO WINLATOR
    // ========================================================

    setupTouchMouse() {

        let lastX = 0;

        let lastY = 0;

        document.addEventListener(
            "touchstart",
            event => {

                if (
                    !event.touches ||
                    !event.touches[0]
                )
                    return;

                lastX =
                    event.touches[0].clientX;

                lastY =
                    event.touches[0].clientY;
            },
            {
                passive: true
            }
        );

        document.addEventListener(
            "touchmove",
            event => {

                if (
                    !event.touches ||
                    !event.touches[0]
                )
                    return;

                const x =
                    event.touches[0].clientX;

                const y =
                    event.touches[0].clientY;

                const dx =
                    x - lastX;

                const dy =
                    y - lastY;

                VirtualPC.mouse.move(
                    dx,
                    dy
                );

                lastX = x;

                lastY = y;
            },
            {
                passive: true
            }
        );

        VirtualPC.mouse.update();
    },

    // ========================================================
    // TECLADO VIRTUAL
    // ========================================================

    keyboard: {

        send(key) {

            console.log(
                "Tecla:",
                key
            );

            VirtualPC.API.sendKeyboard({
                key: key
            });
        },

        create() {

            const keyboard =
                document.getElementById(
                    "virtualKeyboard"
                );

            if (!keyboard)
                return;

            const keys = [

                "ESC",

                "TAB",

                "CTRL",

                "ALT",

                "SHIFT",

                "ENTER",

                "BACKSPACE",

                "↑",

                "↓",

                "←",

                "→"

            ];

            keyboard.innerHTML = "";

            keys.forEach(key => {

                const button =
                    document.createElement(
                        "button"
                    );

                button.textContent =
                    key;

                button.onclick =
                    () => this.send(key);

                keyboard.appendChild(
                    button
                );
            });
        }
    },

    // ========================================================
    // API / BACKEND RUST
    // ========================================================

    API: {

        backendAvailable: false,

        async createVM(config) {

            console.log(
                "Criar VM:",
                config
            );

            return {
                success: true,
                config
            };
        },

        async startVM(id) {

            console.log(
                "Iniciar VM:",
                id
            );

            return {
                success: true
            };
        },

        async stopVM(id) {

            console.log(
                "Desligar VM:",
                id
            );

            return {
                success: true
            };
        },

        async sendMouse(event) {

            console.log(
                "Evento de mouse:",
                event
            );
        },

        async sendKeyboard(event) {

            console.log(
                "Evento de teclado:",
                event
            );
        }
    }
};


// ============================================================
// FUNÇÕES GLOBAIS PARA OS HTML
// ============================================================

function criarVM() {
    VirtualPC.createVM();
}

function abrirVM(id) {
    VirtualPC.openVM(id);
}

function iniciarVM() {
    VirtualPC.startVM();
}

function desligarVM() {
    VirtualPC.stopVM();
}

function apagarVMAtual() {
    VirtualPC.deleteCurrentVM();
}

function apagarTodasVMs() {
    VirtualPC.deleteAllVMs();
}

function abrirConfiguracoes() {
    VirtualPC.openSettings();
}

function voltarParaVMs() {
    VirtualPC.backToVMs();
}

function pressionarTecla(tecla) {
    VirtualPC.keyboard.send(tecla);
}

function cliqueMouse(tipo) {

    if (tipo === "right") {
        VirtualPC.mouse.rightClick();
    } else {
        VirtualPC.mouse.leftClick();
    }
}


// ============================================================
// INICIALIZAÇÃO
// ============================================================

document.addEventListener(
    "DOMContentLoaded",
    () => {

        VirtualPC.renderVMs();

        VirtualPC.loadCurrentVM();

        VirtualPC.setupTouchMouse();

        VirtualPC.keyboard.create();

        console.log(
            `VirtualPC ${VirtualPC.version} iniciado`
        );
    }
);
