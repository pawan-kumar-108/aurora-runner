# 🌐 **AURORA RUNNER - NETLIFY DEPLOYMENT GUIDE**

Complete step-by-step guide to deploy your game to Netlify and make it playable in any browser worldwide!

---

## 📋 **PRE-DEPLOYMENT CHECKLIST**

Before deploying, ensure:

- [x] Game runs locally without errors (`turbo run -w aurora-runner`)
- [x] All features tested and working
- [x] High score save/load works correctly
- [x] No console errors or warnings
- [x] Game performance is smooth (60 FPS)
- [ ] Optional: Custom sprites added
- [ ] Optional: Audio files added

---

## 🚀 **METHOD 1: QUICK DEPLOY (Drag & Drop)**

**Easiest method - Takes 2 minutes!**

### Step 1: Export the Game
```bash
# Navigate to project directory
cd aurora-runner

# Export for web
turbo export aurora-runner
```

This creates optimized WebAssembly build in `target/web/`

### Step 2: Create Netlify Account
1. Go to [netlify.com](https://netlify.com)
2. Click "Sign Up"
3. Use GitHub, GitLab, or Email
4. Verify your email

### Step 3: Deploy
1. Log into Netlify
2. Click "Add new site" → "Deploy manually"
3. Drag `target/web/` folder into the upload zone
4. Wait 30 seconds for deployment
5. Done! Your game is live! 🎉

### Step 4: Custom Domain (Optional)
1. Go to "Site settings" → "Domain management"
2. Change site name: `aurora-runner-yourname.netlify.app`
3. Or add custom domain: `www.aurorarunner.com`

---

## 🔄 **METHOD 2: NETLIFY CLI (Command Line)**

**For developers who prefer terminal workflows**

### Step 1: Install Netlify CLI
```bash
# Using npm
npm install -g netlify-cli

# Or using yarn
yarn global add netlify-cli
```

### Step 2: Authenticate
```bash
netlify login
```
This opens browser for authentication.

### Step 3: Export Game
```bash
cd aurora-runner
turbo export aurora-runner
```

### Step 4: Deploy
```bash
cd target/web
netlify deploy
```

Follow prompts:
- Create new site? **Yes**
- Site name: `aurora-runner` (or your choice)
- Deploy path: `.` (current directory)

### Step 5: Deploy to Production
```bash
netlify deploy --prod
```

Your game is now live! URL will be displayed in terminal.

---

## 🤖 **METHOD 3: GITHUB AUTO-DEPLOY (Best for Teams)**

**Automatic deployment on every code push**

### Step 1: Create GitHub Repository
```bash
cd aurora-runner
git init
git add .
git commit -m "Initial commit: Aurora Runner v1.0"

# Create repo on GitHub.com, then:
git remote add origin https://github.com/yourusername/aurora-runner.git
git push -u origin main
```

### Step 2: Connect to Netlify
1. Go to [netlify.com](https://netlify.com)
2. Click "Add new site" → "Import an existing project"
3. Choose "GitHub"
4. Authorize Netlify
5. Select `aurora-runner` repository

### Step 3: Configure Build Settings
```
Build command: turbo export aurora-runner
Publish directory: target/web
```

Click "Deploy site"

### Step 4: Auto-Deploy is Active!
Now every time you push to GitHub:
```bash
git add .
git commit -m "Added new features"
git push
```
Netlify automatically rebuilds and deploys! ✨

---

## ⚙️ **ADVANCED CONFIGURATION**

### netlify.toml (Optional)
Create this file in project root for advanced settings:

```toml
[build]
  command = "turbo export aurora-runner"
  publish = "target/web"

[build.environment]
  RUST_VERSION = "1.70.0"

[[headers]]
  for = "/*"
  [headers.values]
    X-Frame-Options = "DENY"
    X-XSS-Protection = "1; mode=block"
    X-Content-Type-Options = "nosniff"

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200

[context.production]
  command = "turbo export aurora-runner --release"

[context.deploy-preview]
  command = "turbo export aurora-runner"
```

### Performance Optimization

**1. Enable Asset Optimization**
In Netlify dashboard:
- Settings → Build & deploy → Post processing
- Enable "Bundle CSS" and "Minify CSS"
- Enable "Minify JS"
- Enable "Compress images"

**2. Add Custom Headers**
Settings → Build & deploy → Headers
```
/*
  Cache-Control: public, max-age=31536000, immutable
/*.wasm
  Content-Type: application/wasm
/*.js
  Content-Type: application/javascript
```

**3. Enable HTTP/2 Server Push**
```toml
# In netlify.toml
[[headers]]
  for = "/"
  [headers.values]
    Link = '''
    </aurora-runner.wasm>; rel=preload; as=fetch; crossorigin,
    </main.js>; rel=preload; as=script
    '''
```

---

## 🔒 **SECURITY & PRIVACY**

### Environment Variables
If you add API keys later:

```bash
# Set via CLI
netlify env:set API_KEY "your-secret-key"

# Or in Netlify dashboard
# Settings → Environment variables → Add variable
```

### HTTPS
- Netlify provides free SSL automatically
- Your game will be served over HTTPS
- No configuration needed!

---

## 📊 **MONITORING & ANALYTICS**

### Built-in Netlify Analytics
1. Go to your site dashboard
2. Click "Analytics" tab
3. See:
   - Page views
   - Unique visitors
   - Top pages
   - Bandwidth usage

### Add Google Analytics (Optional)
Add to your HTML head:
```html
<!-- Google Analytics -->
<script async src="https://www.googletagmanager.com/gtag/js?id=G-XXXXXXXXXX"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'G-XXXXXXXXXX');
</script>
```

---

## 🐛 **TROUBLESHOOTING DEPLOYMENT**

### Build Fails

**Error: "Command not found: turbo"**
```bash
# Netlify doesn't have Turbo installed
# Solution: Use GitHub Actions or export locally then drag-drop
```

**Error: "No such file or directory: target/web"**
```bash
# Make sure to run export first
turbo export aurora-runner

# Check files exist
ls target/web
```

### Game Doesn't Load

**White screen after deploy**
1. Open browser console (F12)
2. Check for errors
3. Common issues:
   - WASM file not loading (check path)
   - CORS errors (usually auto-fixed by Netlify)
   - Missing index.html

**Fix: Check Publish Directory**
```bash
# Should be: target/web
# NOT: target/web/aurora-runner
```

### Performance Issues

**Game runs slow on web**
- Enable WASM optimization in export
- Use production build
- Check browser compatibility
- Test on different devices

---

## 🌍 **CUSTOM DOMAINS**

### Free Netlify Subdomain
1. Site settings → Domain management
2. Edit site name
3. Choose: `your-game-name.netlify.app`

### Custom Domain (Paid)
**Option 1: Buy through Netlify**
1. Domain management → "Add a domain"
2. Purchase domain (~$15/year)
3. Auto-configured!

**Option 2: Use Existing Domain**
1. Add custom domain in Netlify
2. Update DNS at your registrar:
   ```
   Type: A
   Name: @
   Value: 75.2.60.5

   Type: CNAME
   Name: www
   Value: your-site.netlify.app
   ```
3. Wait for DNS propagation (up to 48 hours)

---

## 📱 **MOBILE OPTIMIZATION**

### PWA (Progressive Web App)
Make your game installable on mobile:

**1. Create manifest.json**
```json
{
  "name": "Aurora Runner",
  "short_name": "Aurora",
  "start_url": "/",
  "display": "fullscreen",
  "orientation": "landscape",
  "theme_color": "#1a0033",
  "background_color": "#0f4c75",
  "icons": [
    {
      "src": "/icon-192.png",
      "sizes": "192x192",
      "type": "image/png"
    },
    {
      "src": "/icon-512.png",
      "sizes": "512x512",
      "type": "image/png"
    }
  ]
}
```

**2. Add to index.html**
```html
<link rel="manifest" href="/manifest.json">
<meta name="theme-color" content="#1a0033">
```

**3. Test Installation**
- Open on mobile browser
- Click "Add to Home Screen"
- Game launches like native app!

---

## 🎯 **DEPLOYMENT CHECKLIST**

Before going live, verify:

### Functionality
- [ ] Game loads without errors
- [ ] Controls work (keyboard + touch)
- [ ] Audio plays (if added)
- [ ] High scores save correctly
- [ ] Game over screen works
- [ ] Can restart game

### Performance
- [ ] Loads in < 3 seconds
- [ ] Runs at 60 FPS
- [ ] No memory leaks (play for 10 minutes)
- [ ] Works on mobile devices

### Polish
- [ ] Favicon set
- [ ] Page title correct
- [ ] Meta tags for sharing
- [ ] Responsive design
- [ ] Proper error handling

### SEO & Sharing
- [ ] Meta description
- [ ] Open Graph tags
- [ ] Twitter Card tags
- [ ] Sitemap (optional)

---

## 📢 **SHARING YOUR GAME**

### Social Media Preview
Add to HTML `<head>`:
```html
<!-- Open Graph (Facebook, LinkedIn) -->
<meta property="og:title" content="Aurora Runner - Christmas Game">
<meta property="og:description" content="Soar through the Northern Lights in this magical endless runner!">
<meta property="og:image" content="https://yoursite.com/preview.png">
<meta property="og:url" content="https://yoursite.com">

<!-- Twitter Card -->
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:title" content="Aurora Runner">
<meta name="twitter:description" content="A Christmas endless runner through the Northern Lights">
<meta name="twitter:image" content="https://yoursite.com/preview.png">
```

### QR Code
Generate QR code for your game URL:
- Use [qr-code-generator.com](https://www.qr-code-generator.com)
- Print on business cards
- Share at events

---

## 💰 **COST BREAKDOWN**

### Free Tier (Perfect for This Project)
- Netlify hosting: **FREE**
- 100 GB bandwidth/month
- 300 build minutes/month
- Automatic SSL
- Custom subdomains

### What You Pay For (Optional)
- Custom domain: ~$15/year
- Netlify Analytics: $9/month (optional)
- Increased bandwidth: $20/month (if you go viral!)

**For this project: $0-15/year** 🎉

---

## 🚀 **QUICK DEPLOY COMMANDS**

Copy-paste ready commands:

```bash
# Full deployment workflow
cd aurora-runner
turbo export aurora-runner
cd target/web
netlify deploy --prod

# Or with git
git add .
git commit -m "Deploy: Aurora Runner v1.0"
git push origin main
# (Auto-deploys if connected to Netlify)

# Update after changes
turbo export aurora-runner && cd target/web && netlify deploy --prod && cd ../..
```

---

## 🎓 **LEARNING RESOURCES**

- [Netlify Documentation](https://docs.netlify.com)
- [Turbo Export Guide](https://docs.turbo.computer/learn/guides/web-publishing)
- [WebAssembly Performance](https://web.dev/webassembly/)
- [PWA Guide](https://web.dev/progressive-web-apps/)

---

## 🏁 **FINAL STEPS**

1. ✅ Export game: `turbo export aurora-runner`
2. ✅ Test locally: Open `target/web/index.html`
3. ✅ Deploy to Netlify (choose method above)
4. ✅ Test live site on multiple devices
5. ✅ Share with friends!
6. ✅ Submit to game competition

---

## 🎉 **YOU'RE LIVE!**

Congratulations! Your game is now playable worldwide.

**Share your URL:**
`https://aurora-runner.netlify.app`

**Next steps:**
- Add to portfolio
- Share on social media
- Submit to game jams
- Get feedback
- Iterate and improve!

---

**Made something awesome? Let the world play it!** 🌟

*Happy deploying!* 🚀