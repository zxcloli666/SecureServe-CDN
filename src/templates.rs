pub const INDEX_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>SecureServe CDN</title>
<style>
*{margin:0;padding:0;box-sizing:border-box}
body{
  min-height:100vh;
  display:flex;align-items:center;justify-content:center;
  background:#08080e;
  background-image:
    radial-gradient(ellipse 80% 60% at 50% 0%, rgba(99,102,241,.15) 0%, transparent 50%),
    radial-gradient(ellipse 60% 50% at 80% 50%, rgba(168,85,247,.08) 0%, transparent 50%),
    radial-gradient(ellipse 60% 50% at 20% 80%, rgba(59,130,246,.08) 0%, transparent 50%);
  font-family:-apple-system,BlinkMacSystemFont,'SF Pro Display','SF Pro Text','Helvetica Neue',sans-serif;
  color:#f0f0f5;
  overflow:hidden;
}
.glass{
  position:relative;
  padding:56px 64px;
  border-radius:28px;
  background:rgba(255,255,255,.04);
  backdrop-filter:blur(60px) saturate(1.4);
  -webkit-backdrop-filter:blur(60px) saturate(1.4);
  border:1px solid rgba(255,255,255,.08);
  box-shadow:
    0 0 0 0.5px rgba(255,255,255,.06) inset,
    0 24px 80px rgba(0,0,0,.4),
    0 2px 16px rgba(0,0,0,.2);
  text-align:center;
  animation:fadeUp .8s cubic-bezier(.16,1,.3,1) both;
}
.glass::before{
  content:'';position:absolute;inset:0;border-radius:28px;
  background:linear-gradient(135deg,rgba(255,255,255,.06) 0%,transparent 50%);
  pointer-events:none;
}
h1{
  font-size:2rem;font-weight:600;letter-spacing:-.02em;
  background:linear-gradient(135deg,#e0e0ff 0%,#a0a0cc 100%);
  -webkit-background-clip:text;-webkit-text-fill-color:transparent;
  background-clip:text;
}
p{
  margin-top:12px;font-size:.95rem;font-weight:400;
  color:rgba(255,255,255,.45);letter-spacing:.01em;
}
.dot{
  display:inline-block;width:8px;height:8px;border-radius:50%;
  background:#34d399;box-shadow:0 0 12px rgba(52,211,153,.4);
  margin-right:10px;vertical-align:middle;
  animation:pulse 2s ease-in-out infinite;
}
@keyframes fadeUp{from{opacity:0;transform:translateY(20px)}to{opacity:1;transform:translateY(0)}}
@keyframes pulse{0%,100%{opacity:1}50%{opacity:.4}}
</style>
</head>
<body>
<div class="glass">
  <h1><span class="dot"></span>SecureServe CDN</h1>
  <p>File storage &amp; delivery service</p>
</div>
</body>
</html>"##;

pub const NOT_FOUND_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>404 &mdash; Not Found</title>
<style>
*{margin:0;padding:0;box-sizing:border-box}
body{
  min-height:100vh;
  display:flex;align-items:center;justify-content:center;
  background:#08080e;
  background-image:
    radial-gradient(ellipse 80% 60% at 50% 0%, rgba(239,68,68,.1) 0%, transparent 50%),
    radial-gradient(ellipse 60% 50% at 80% 50%, rgba(168,85,247,.08) 0%, transparent 50%);
  font-family:-apple-system,BlinkMacSystemFont,'SF Pro Display','SF Pro Text','Helvetica Neue',sans-serif;
  color:#f0f0f5;overflow:hidden;
}
.glass{
  position:relative;padding:56px 64px;border-radius:28px;
  background:rgba(255,255,255,.04);
  backdrop-filter:blur(60px) saturate(1.4);
  -webkit-backdrop-filter:blur(60px) saturate(1.4);
  border:1px solid rgba(255,255,255,.08);
  box-shadow:0 0 0 .5px rgba(255,255,255,.06) inset,0 24px 80px rgba(0,0,0,.4),0 2px 16px rgba(0,0,0,.2);
  text-align:center;
  animation:fadeUp .8s cubic-bezier(.16,1,.3,1) both;
}
.glass::before{
  content:'';position:absolute;inset:0;border-radius:28px;
  background:linear-gradient(135deg,rgba(255,255,255,.06) 0%,transparent 50%);
  pointer-events:none;
}
.code{
  font-size:5rem;font-weight:700;letter-spacing:-.04em;
  background:linear-gradient(135deg,rgba(255,255,255,.15) 0%,rgba(255,255,255,.05) 100%);
  -webkit-background-clip:text;-webkit-text-fill-color:transparent;
  background-clip:text;line-height:1;
}
h1{
  margin-top:16px;font-size:1.4rem;font-weight:600;letter-spacing:-.01em;
  color:rgba(255,255,255,.7);
}
p{
  margin-top:8px;font-size:.9rem;color:rgba(255,255,255,.35);
}
@keyframes fadeUp{from{opacity:0;transform:translateY(20px)}to{opacity:1;transform:translateY(0)}}
</style>
</head>
<body>
<div class="glass">
  <div class="code">404</div>
  <h1>File Not Found</h1>
  <p>The requested resource does not exist on this server.</p>
</div>
</body>
</html>"##;

pub const ADMIN_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>SecureServe CDN &mdash; Admin</title>
<style>
*{margin:0;padding:0;box-sizing:border-box}
:root{
  --bg:#08080e;
  --glass:rgba(255,255,255,.04);
  --glass-border:rgba(255,255,255,.08);
  --glass-highlight:rgba(255,255,255,.06);
  --text:#f0f0f5;
  --text-dim:rgba(255,255,255,.45);
  --text-muted:rgba(255,255,255,.25);
  --accent:rgb(99,102,241);
  --accent-glow:rgba(99,102,241,.3);
  --danger:rgb(239,68,68);
  --success:rgb(52,211,153);
  --radius:16px;
  --radius-sm:10px;
}
body{
  min-height:100vh;
  background:var(--bg);
  background-image:
    radial-gradient(ellipse 80% 60% at 50% 0%,rgba(99,102,241,.12) 0%,transparent 50%),
    radial-gradient(ellipse 60% 50% at 80% 50%,rgba(168,85,247,.06) 0%,transparent 50%),
    radial-gradient(ellipse 60% 50% at 20% 80%,rgba(59,130,246,.06) 0%,transparent 50%);
  font-family:-apple-system,BlinkMacSystemFont,'SF Pro Display','SF Pro Text','Helvetica Neue',sans-serif;
  color:var(--text);
  display:flex;align-items:center;justify-content:center;
  padding:24px;
}
.glass{
  position:relative;
  border-radius:var(--radius);
  background:var(--glass);
  backdrop-filter:blur(60px) saturate(1.4);
  -webkit-backdrop-filter:blur(60px) saturate(1.4);
  border:1px solid var(--glass-border);
  box-shadow:0 0 0 .5px var(--glass-highlight) inset,0 24px 80px rgba(0,0,0,.4);
}
.glass::before{
  content:'';position:absolute;inset:0;border-radius:inherit;
  background:linear-gradient(135deg,var(--glass-highlight) 0%,transparent 50%);
  pointer-events:none;
}
.container{
  width:100%;max-width:640px;
  animation:fadeUp .6s cubic-bezier(.16,1,.3,1) both;
}

/* Login */
#login-view{padding:48px 40px;}
#login-view h1{
  font-size:1.5rem;font-weight:600;letter-spacing:-.02em;text-align:center;
  background:linear-gradient(135deg,#e0e0ff,#a0a0cc);
  -webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text;
}
#login-view p{text-align:center;color:var(--text-dim);font-size:.85rem;margin-top:8px;}
.form-group{margin-top:24px;}
.form-group label{display:block;font-size:.75rem;font-weight:500;color:var(--text-dim);margin-bottom:6px;text-transform:uppercase;letter-spacing:.06em;}
input[type="password"],input[type="text"]{
  width:100%;padding:12px 16px;
  background:rgba(255,255,255,.05);
  border:1px solid rgba(255,255,255,.1);
  border-radius:var(--radius-sm);
  color:var(--text);font-size:.95rem;
  outline:none;transition:border-color .2s,box-shadow .2s;
}
input:focus{border-color:var(--accent);box-shadow:0 0 0 3px var(--accent-glow);}
.btn{
  display:inline-flex;align-items:center;justify-content:center;gap:6px;
  padding:10px 20px;border-radius:var(--radius-sm);
  font-size:.85rem;font-weight:500;
  cursor:pointer;border:none;transition:all .2s;
  position:relative;
}
.btn-primary{
  background:var(--accent);color:#fff;width:100%;margin-top:20px;
  box-shadow:0 2px 12px var(--accent-glow);
}
.btn-primary:hover{filter:brightness(1.15);transform:translateY(-1px);}
.btn-sm{padding:6px 14px;font-size:.78rem;}
.btn-danger{background:rgba(239,68,68,.15);color:var(--danger);border:1px solid rgba(239,68,68,.2);}
.btn-danger:hover{background:rgba(239,68,68,.25);}
.btn-ghost{background:rgba(255,255,255,.06);color:var(--text-dim);border:1px solid rgba(255,255,255,.08);}
.btn-ghost:hover{background:rgba(255,255,255,.1);color:var(--text);}

/* Dashboard */
#dashboard-view{display:none;}
.dash-header{
  padding:24px 32px;
  display:flex;align-items:center;justify-content:space-between;
  border-bottom:1px solid rgba(255,255,255,.06);
}
.dash-header h1{
  font-size:1.1rem;font-weight:600;letter-spacing:-.01em;
  background:linear-gradient(135deg,#e0e0ff,#a0a0cc);
  -webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text;
}
.dash-body{padding:24px 32px;}
.section-title{
  display:flex;align-items:center;justify-content:space-between;
  margin-bottom:16px;
}
.section-title h2{font-size:.85rem;font-weight:500;color:var(--text-dim);text-transform:uppercase;letter-spacing:.06em;}

/* Token list */
.token-list{display:flex;flex-direction:column;gap:8px;}
.token-card{
  display:flex;align-items:center;gap:12px;
  padding:14px 16px;
  background:rgba(255,255,255,.03);
  border:1px solid rgba(255,255,255,.06);
  border-radius:var(--radius-sm);
  transition:background .15s;
}
.token-card:hover{background:rgba(255,255,255,.06);}
.token-info{flex:1;min-width:0;}
.token-name{font-size:.9rem;font-weight:500;}
.token-value{
  font-size:.72rem;font-family:'SF Mono',SFMono-Regular,Consolas,'Liberation Mono',Menlo,monospace;
  color:var(--text-muted);margin-top:2px;
  white-space:nowrap;overflow:hidden;text-overflow:ellipsis;
  cursor:pointer;
}
.token-date{font-size:.7rem;color:var(--text-muted);white-space:nowrap;}
.empty-state{
  text-align:center;padding:40px 20px;
  color:var(--text-muted);font-size:.85rem;
}

/* Create form */
.create-row{
  display:flex;gap:8px;margin-top:16px;
}
.create-row input{flex:1;}

/* Toast */
.toast{
  position:fixed;bottom:24px;left:50%;transform:translateX(-50%) translateY(20px);
  padding:10px 20px;border-radius:var(--radius-sm);
  background:rgba(255,255,255,.08);
  backdrop-filter:blur(20px);
  border:1px solid rgba(255,255,255,.1);
  font-size:.85rem;color:var(--text);
  opacity:0;transition:all .3s cubic-bezier(.16,1,.3,1);
  pointer-events:none;z-index:100;
}
.toast.visible{opacity:1;transform:translateX(-50%) translateY(0);}
.toast.error{border-color:rgba(239,68,68,.3);color:var(--danger);}
.toast.success{border-color:rgba(52,211,153,.3);color:var(--success);}

.error-msg{color:var(--danger);font-size:.8rem;margin-top:8px;text-align:center;display:none;}
@keyframes fadeUp{from{opacity:0;transform:translateY(20px)}to{opacity:1;transform:translateY(0)}}
@keyframes spin{to{transform:rotate(360deg)}}
</style>
</head>
<body>

<div class="container">
  <!-- Login -->
  <div id="login-view" class="glass">
    <h1>SecureServe CDN</h1>
    <p>Enter admin token to continue</p>
    <div class="form-group">
      <label>Admin Token</label>
      <input type="password" id="admin-token" placeholder="Enter token..." autocomplete="off">
    </div>
    <div class="error-msg" id="login-error"></div>
    <button class="btn btn-primary" id="login-btn">Authenticate</button>
  </div>

  <!-- Dashboard -->
  <div id="dashboard-view" class="glass">
    <div class="dash-header">
      <h1>Upload Tokens</h1>
      <button class="btn btn-ghost btn-sm" id="logout-btn">Logout</button>
    </div>
    <div class="dash-body">
      <div class="section-title">
        <h2>Active Tokens</h2>
        <span id="token-count" style="font-size:.75rem;color:rgba(255,255,255,.25);">0</span>
      </div>
      <div class="token-list" id="token-list">
        <div class="empty-state">No tokens yet</div>
      </div>
      <div class="create-row">
        <input type="text" id="new-token-name" placeholder="Token name...">
        <button class="btn btn-primary btn-sm" id="create-btn" style="width:auto;margin:0;">Create</button>
      </div>
    </div>
  </div>
</div>

<div class="toast" id="toast"></div>

<script>
(function(){
  const $ = function(s){ return document.querySelector(s); };
  let adminToken = sessionStorage.getItem('adminToken') || '';

  const api = async function(method, path, body) {
    const opts = { method: method, headers: { 'Authorization': 'Bearer ' + adminToken, 'Content-Type': 'application/json' } };
    if (body) opts.body = JSON.stringify(body);
    const r = await fetch(path, opts);
    const data = await r.json();
    if (!r.ok) throw new Error(data.error || 'Request failed');
    return data;
  };

  const toast = function(msg, type) {
    const el = $('#toast');
    el.textContent = msg;
    el.className = 'toast ' + (type || '') + ' visible';
    setTimeout(function(){ el.classList.remove('visible'); }, 2500);
  };

  const formatDate = function(ts) {
    const d = new Date(ts * 1000);
    return d.toLocaleDateString('en-US', { month:'short', day:'numeric', year:'numeric' });
  };

  function buildTokenCard(t) {
    const card = document.createElement('div');
    card.className = 'token-card';
    card.dataset.id = t.id;

    const info = document.createElement('div');
    info.className = 'token-info';

    const name = document.createElement('div');
    name.className = 'token-name';
    name.textContent = t.name;

    const value = document.createElement('div');
    value.className = 'token-value';
    value.textContent = t.token;
    value.title = t.token;
    value.addEventListener('click', function() {
      navigator.clipboard.writeText(t.token);
      toast('Copied to clipboard', 'success');
    });

    info.appendChild(name);
    info.appendChild(value);

    const date = document.createElement('div');
    date.className = 'token-date';
    date.textContent = formatDate(t.created_at);

    const delBtn = document.createElement('button');
    delBtn.className = 'btn btn-danger btn-sm';
    delBtn.textContent = 'Delete';
    delBtn.addEventListener('click', async function() {
      if (!confirm('Delete this token?')) return;
      try {
        await api('DELETE', '/api/tokens/' + t.id);
        toast('Token deleted', 'success');
        renderTokens();
      } catch(e) { toast(e.message, 'error'); }
    });

    card.appendChild(info);
    card.appendChild(date);
    card.appendChild(delBtn);
    return card;
  }

  const renderTokens = async function() {
    try {
      const tokens = await api('GET', '/api/tokens');
      const list = $('#token-list');
      $('#token-count').textContent = tokens.length;

      while (list.firstChild) list.removeChild(list.firstChild);

      if (!tokens.length) {
        const empty = document.createElement('div');
        empty.className = 'empty-state';
        empty.textContent = 'No tokens yet';
        list.appendChild(empty);
        return;
      }

      tokens.forEach(function(t) {
        list.appendChild(buildTokenCard(t));
      });
    } catch(e) { toast(e.message, 'error'); }
  };

  const showDashboard = function() {
    $('#login-view').style.display = 'none';
    $('#dashboard-view').style.display = 'block';
    renderTokens();
  };

  const showLogin = function() {
    adminToken = '';
    sessionStorage.removeItem('adminToken');
    $('#login-view').style.display = 'block';
    $('#dashboard-view').style.display = 'none';
    $('#admin-token').value = '';
  };

  const doLogin = async function() {
    adminToken = $('#admin-token').value.trim();
    if (!adminToken) return;
    try {
      await api('POST', '/api/auth/verify');
      sessionStorage.setItem('adminToken', adminToken);
      showDashboard();
    } catch(e) {
      const err = $('#login-error');
      err.textContent = e.message;
      err.style.display = 'block';
      adminToken = '';
    }
  };

  $('#login-btn').addEventListener('click', doLogin);
  $('#admin-token').addEventListener('keydown', function(e) { if (e.key === 'Enter') doLogin(); });
  $('#logout-btn').addEventListener('click', showLogin);

  const doCreate = async function() {
    const nameInput = $('#new-token-name');
    const name = nameInput.value.trim();
    if (!name) return;
    try {
      await api('POST', '/api/tokens', { name: name });
      nameInput.value = '';
      toast('Token created', 'success');
      renderTokens();
    } catch(e) { toast(e.message, 'error'); }
  };

  $('#create-btn').addEventListener('click', doCreate);
  $('#new-token-name').addEventListener('keydown', function(e) { if (e.key === 'Enter') doCreate(); });

  if (adminToken) {
    api('POST', '/api/auth/verify').then(showDashboard).catch(showLogin);
  }
})();
</script>
</body>
</html>"##;
