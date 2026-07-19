<div align="center">

<img src="assets/banner.png" width="100%" alt="stress-attack illustrative banner" />

# stress-attack

**Teste de carga HTTP autorizado** · visual dark · Rust

[![Rust](https://img.shields.io/badge/Rust-1.70+-0a0a0a?style=for-the-badge&logo=rust&logoColor=ef4444)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-7f1d1d?style=for-the-badge)](LICENSE)
[![Autor](https://img.shields.io/badge/Autor-ProezaDEV-ef4444?style=for-the-badge&logo=github&logoColor=white)](https://github.com/ProezaDEV)
[![Email](https://img.shields.io/badge/Email-proezadev%40gmail.com-0a0a0a?style=for-the-badge&logo=gmail&logoColor=ef4444)](mailto:proezadev@gmail.com)

</div>

---

> [!WARNING]
> **Somente alvos autorizados.**  
> Use apenas em sistemas **seus** ou com **permissão explícita**.  
> Uso contra terceiros sem autorização é ilegal.

## Sobre

Ferramenta de **stress / load test HTTP** criada por **ProezaDEV**.  
Serve para medir como **seu** serviço se comporta sob concorrência.

| Recurso | Detalhe |
| :--- | :--- |
| Load HTTP | GET / POST / PUT / HEAD em paralelo |
| Métricas | ok / falha / bytes / req/s |
| Segurança | exige `--i-am-authorized` para rodar |

## Contato

- **Autor:** [ProezaDEV](https://github.com/ProezaDEV)
- **Email:** [proezadev@gmail.com](mailto:proezadev@gmail.com)

## Requisitos

- Rust 1.70+
- URL HTTP(S) que você tem permissão para testar

## Instalação

```bash
git clone https://github.com/ProezaDEV/stress-attack.git
cd stress-attack
cargo build --release
```

Binário: `target/release/stress` (Windows: `stress.exe`)

---

## Uso

> Sempre inclua `--i-am-authorized`. Sem essa flag, o programa **não executa**.

### Padrão básico
```bash
./target/release/stress \
  --url https://seu-servidor.local/health \
  --concurrency 32 \
  --requests 1000 \
  --method GET \
  --i-am-authorized
```

### Load leve (smoke test)
```bash
./target/release/stress \
  --url https://seu-servidor.local/ \
  -c 8 -r 200 -m GET --timeout 5 \
  --i-am-authorized
```

### Load médio (API)
```bash
./target/release/stress \
  --url https://api.seu-dominio.local/v1/status \
  -c 32 -r 2000 -m GET \
  --i-am-authorized
```

### Load alto (stress)
```bash
./target/release/stress \
  --url https://seu-servidor.local/health \
  -c 100 -r 10000 -m GET --timeout 15 \
  --i-am-authorized
```

### POST / PUT / HEAD
```bash
./target/release/stress --url https://api.seu-dominio.local/v1/echo -c 20 -r 500 -m POST --i-am-authorized
./target/release/stress --url https://api.seu-dominio.local/v1/items/1 -c 16 -r 400 -m PUT --i-am-authorized
./target/release/stress --url https://seu-servidor.local/ -c 40 -r 3000 -m HEAD --i-am-authorized
```

### Windows (PowerShell)
```powershell
.\target\release\stress.exe `
  --url https://seu-servidor.local/health `
  -c 32 -r 1000 -m GET `
  --i-am-authorized
```

---

## Flags

| Flag | Descrição |
| :--- | :--- |
| `--url` | URL do alvo autorizado |
| `-c, --concurrency` | Workers em paralelo (padrão: `32`) |
| `-r, --requests` | Total de requests (padrão: `1000`) |
| `-m, --method` | `GET` / `POST` / `PUT` / `HEAD` |
| `--timeout` | Timeout por request (padrão: `10`) |
| `--i-am-authorized` | **Obrigatório** — confirma autorização |

---

## Aviso legal

### Uso permitido
- Testar infraestrutura **sua**
- Lab / staging com permissão
- Aprendizado de performance

### Uso proibido
- Atacar sistemas sem autorização
- Derrubar serviços de terceiros
- Qualquer atividade ilegal

## Autor

**ProezaDEV** · [proezadev@gmail.com](mailto:proezadev@gmail.com)  
Criação própria · Full Stack · Ethical Hacking

---

<div align="center">

**stress-attack** · by ProezaDEV

</div>
