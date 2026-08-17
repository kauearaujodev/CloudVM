// ============================================================
// VirtualPC - Gerenciador da interface
// ============================================================

const VirtualPC = {

    salvarVM(vm) {
        localStorage.setItem(
            "virtualPC_vm",
            JSON.stringify(vm)
        );
    },

    obterVM() {
        const dados = localStorage.getItem("virtualPC_vm");

        if (!dados) {
            return null;
        }

        try {
            return JSON.parse(dados);
        } catch {
            return null;
        }
    },

    excluirVM() {
        localStorage.removeItem("virtualPC_vm");
    },

    abrirVM() {
        window.location.href = "vm.html";
    },

    criarVM(config) {

        const vm = {
            id: Date.now(),

            nome: config.nome,

            cpu: Number(config.cpu),

            ram: Number(config.ram),

            storage: Number(config.storage),

            status: "desligada",

            criadaEm: new Date().toISOString()
        };

        this.salvarVM(vm);

        return vm;
    }
};


// ============================================================
// Criar VM
// ============================================================

function criarVirtualMachine() {

    const nome =
        document.getElementById("vmNome")?.value.trim();

    const cpu =
        document.getElementById("vmCPU")?.value;

    const ram =
        document.getElementById("vmRAM")?.value;

    const storage =
        document.getElementById("vmStorage")?.value;

    if (!nome) {
        alert("Digite um nome para a VM.");
        return;
    }

    const vm = VirtualPC.criarVM({
        nome,
        cpu,
        ram,
        storage
    });

    console.log("VM criada:", vm);

    window.location.href = "minhas-vms.html";
}


// ============================================================
// Carregar VM
// ============================================================

function carregarVM() {

    const vm = VirtualPC.obterVM();

    if (!vm) {
        return;
    }

    const nome =
        document.getElementById("vmName");

    if (nome) {
        nome.textContent = vm.nome;
    }

    const ram =
        document.getElementById("vmRam");

    if (ram) {
        ram.textContent = `${vm.ram} GB`;
    }

    const cpu =
        document.getElementById("vmCpu");

    if (cpu) {
        cpu.textContent = `${vm.cpu} núcleos`;
    }

    const storage =
        document.getElementById("vmStorage");

    if (storage) {
        storage.textContent =
            `${vm.storage} GB`;
    }
}


// ============================================================
// Abrir VM
// ============================================================

function abrirMinhaVM() {

    const vm = VirtualPC.obterVM();

    if (!vm) {
        alert("Nenhuma VM foi criada.");
        return;
    }

    window.location.href = "vm.html";
}


// ============================================================
// Configurações
// ============================================================

function limparVM() {

    if (
        confirm(
            "Deseja apagar a VM salva neste dispositivo?"
        )
    ) {

        VirtualPC.excluirVM();

        alert("VM removida.");

        window.location.href =
            "minhas-vms.html";
    }
}

document.addEventListener(
    "DOMContentLoaded",
    carregarVM
);
