# خطة: تنفيذ مشروع Floating MD

> **تاريخ الإنشاء:** 2026-01-23  
> **تاريخ التحديث:** 2026-01-23 01:49  
> **الحالة:** ✅ جاهزة للموافقة النهائية  
> **المطور:** احمد صالح  
> **النموذج الحالي:** Gemini 3 Flash

---

## 📋 ملخص المشروع

**اسم المشروع:** Floating MD (The Desktop Bridge)

**الوصف:**  
شريط أدوات عائم (Utility Bar) مخصص لنظام ويندوز، يعمل كوسيط بين تطبيقات الدردشة مع الذكاء الاصطناعي وبين بيئة العمل المحلية (Editors, Browsers). يوفر تحكم كامل في عمليات النسخ، اللصق، ومعالجة نصوص الماركداون واللغة العربية باستخدام الماوس فقط.

**التقنيات المعتمدة:**
- **App Framework:** Tauri 2.0
- **Backend Language:** Rust (2024 Edition)
- **Frontend:** React 18 + TypeScript
- **Styling:** Tailwind CSS 4.x
- **State Management:** Zustand
- **Database:** SQLite + FTS5
- **Markdown Engine:** unified + remark + rehype + Shiki
- **Build Tool:** Vite 6.x
- **Package Manager:** pnpm

---

## 🎯 الأهداف الرئيسية

1. ✅ **نسخ ولصق عالمي** دون سرقة التركيز (Focus Stealing)
2. ✅ **مدير حافظة** مع سجل وإمكانية البحث والتثبيت
3. ✅ **محرر ماركداون** مع معاينة حية ودعم كامل للعربية (RTL)
4. ✅ **تنظيف ذكي** لنصوص AI (Smart Scrub)
5. ✅ **واجهة عائمة** مع إخفاء تلقائي ودعم multi-monitor
6. ✅ **أداء عالي** (حجم صغير، بدء سريع، استهلاك ذاكرة منخفض)

---

## ✅ Approved Technical Decisions

> [!NOTE]
> **القرارات التقنية التالية تم اعتمادها من المطور:**  
> **تاريخ الموافقة:** 2026-01-23 01:49

### 1️⃣ Frontend Framework: React 18

**القرار المعتمد:** استخدام **React 18** كـ Frontend Framework

**المبررات:**
- نظام بيئي ضخم ومستقر (millions of developers)
- مكتبات جاهزة لكل الاحتياجات
- سهولة التوظيف والصيانة المستقبلية
- حصل على 2/3 أصوات في التقارير التقنية
- فرق الحجم (37 KB) غير حرج في Desktop App

**البدائل المرفوضة:** SolidJS 2.x (رغم الأداء العالي، النظام البيئي محدود)

---

### 2️⃣ Window Vibrancy: Acrylic Effect + Fallback

**القرار المعتمد:** **الخيار A** - تفعيل التأثير الزجاجي مع Fallback

**التنفيذ:**
- استخدام `tauri-plugin-window-vibrancy` للتأثير الزجاجي
- Fallback تلقائي لخلفية صلبة على الأنظمة غير المدعومة
- Target: Windows 11 Mica / Windows 10 Acrylic

**المبررات:**
- مظهر Premium يطابق الـ Reference (Manus)
- التوافق مع Windows 11 Design Language
- الـ Fallback يضمن العمل على جميع الإصدارات

**البدائل المرفوضة:** الخيار B (خلفية صلبة فقط)

---

### 3️⃣ Smart Scrub: Manual Activation

**القرار المعتمد:** **التفعيل اليدوي (Manual)** عبر زر Sparkle ✨

**التنفيذ:**
- Sparkle button يظهر فقط لما يكتشف AI text patterns
- المستخدم يضغط الزر لتنظيف النص
- Toast notification مع Undo option

**المبررات:**
- أمان أعلى (صفر احتمال حذف بيانات مهمة)
- ثقة المستخدم في التطبيق
- UX best practice للأفعال المدمرة (Destructive actions)
- شفافية كاملة

**البدائل المرفوضة:** التنظيف التلقائي (Auto Scrub)

---

## 📐 Proposed Changes

### المرحلة 1: إعداد البيئة والهيكل القياسي

**النموذج المناسب:** Gemini 3 Flash (مهام سريعة وبسيطة)

#### المهام:

- [ ] 1.1: إنشاء الهيكل القياسي للمشروع
  
  **حسب MASTER_CONSTITUTION - القسم 3:**
  ```bash
  mkdir floating-md
  cd floating-md
  
  # الهيكل الإلزامي
  mkdir docs plans scripts Backups
  
  # إنشاء الملفات الحاكمة
  # PROJECT_CONTEXT.md ✅ (تم إنشاؤه)
  # MASTER_CONSTITUTION.md (نسخ من docs)
  ```

- [ ] 1.2: تهيئة Git Repository (إلزامي من اللحظة الأولى)
  ```bash
  git init
  git add .
  git commit -m "Initial commit - Project structure setup v0.1.0"
  ```

- [ ] 1.3: نسخ الملفات الحاكمة إلى المشروع
  ```bash
  cp plane/docs/MASTER_CONSTITUTION.md ./docs/
  cp plane/docs/SOTA_Models_2026.md ./docs/
  ```

- [ ] 1.4: تهيئة مشروع Tauri + React
  ```bash
  pnpm create tauri-app --template react-ts
  ```

- [ ] 1.5: تثبيت المكتبات الأساسية
  ```bash
  # State Management
  pnpm add zustand
  
  # Styling
  pnpm add -D tailwindcss postcss autoprefixer
  pnpm dlx tailwindcss init -p
  
  # Markdown Engine
  pnpm add unified remark-parse remark-rehype rehype-stringify
  pnpm add remark-gfm rehype-raw
  pnpm add shiki
  
  # Tauri Plugins
  pnpm add @tauri-apps/plugin-clipboard-manager
  pnpm add @tauri-apps/plugin-window
  ```

- [ ] 1.6: إنشاء هيكل المجلدات حسب Clean Architecture
  ```
  floating-md/
  ├── docs/                    # ✅ Documentation
  │   ├── MASTER_CONSTITUTION.md
  │   ├── SOTA_Models_2026.md
  │   └── API_REFERENCE.md
  ├── plans/                   # ✅ Implementation Plans
  │   └── 01-implementation-plan.md
  ├── scripts/                 # ✅ DevOps Scripts
  │   ├── dev.ps1             # Development start
  │   ├── build.ps1           # Production build
  │   ├── clean.ps1           # Cleanup
  │   └── backup.ps1          # Versioned backup
  ├── Backups/                 # ✅ Versioned Backups
  │   └── .gitkeep
  ├── src-tauri/              # Rust Backend
  │   ├── src/
  │   │   ├── main.rs
  │   │   ├── window/
  │   │   │   ├── mod.rs
  │   │   │   ├── noactivate.rs
  │   │   │   └── docking.rs
  │   │   ├── input/
  │   │   │   ├── mod.rs
  │   │   │   ├── keyboard.rs
  │   │   │   └── clipboard.rs
  │   │   └── db/
  │   │       ├── mod.rs
  │   │       ├── schema.rs
  │   │       └── queries.rs
  │   └── Cargo.toml
  ├── src/                    # React Frontend
  │   ├── components/
  │   │   ├── Dock/
  │   │   │   ├── Dock.tsx
  │   │   │   └── DockItem.tsx
  │   │   ├── Bubble/
  │   │   │   └── HoverBubble.tsx
  │   │   └── Editor/
  │   │       └── MarkdownEditor.tsx
  │   ├── hooks/
  │   │   ├── useClipboard.ts
  │   │   └── useDocking.ts
  │   ├── stores/
  │   │   └── clipboardStore.ts
  │   ├── utils/
  │   │   ├── bidi.ts
  │   │   └── markdown.ts
  │   ├── App.tsx
  │   └── main.tsx
  ├── public/
  │   └── fonts/              # الخطوط المضمنة
  │       ├── GeistSans-Variable.woff2
  │       ├── GeistMono-Variable.woff2
  │       └── IBMPlexSansArabic-Regular.woff2
  ├── PROJECT_CONTEXT.md      # ✅ ذاكرة المشروع
  ├── .gitignore
  ├── package.json
  ├── vite.config.ts
  ├── tailwind.config.js
  └── tsconfig.json
  ```

- [ ] 1.7: إنشاء سكربتات الصيانة (محمولة - Portable)
  
  **ملاحظة:** السكربتات يجب أن تعمل من أي مسار طالما هي داخل المشروع

**✅ مكتملة بواسطة:** [سيُملأ بعد التنفيذ]  
**📅 التاريخ:** [سيُملأ بعد التنفيذ]

---

### المرحلة 2: بناء Backend - Rust Layer

**النموذج المناسب:** Claude Opus 4.5 (Thinking)

**السبب من SOTA_Models_2026:**
- Champion في Software Engineering (80.9% SWE-Bench Verified)
- الأفضل لـ Multi-file refactoring و System design
- Superior safety - مهم جداً لـ Win32 APIs الخطرة

#### المهام:

- [ ] 2.1: إعداد Cargo.toml مع المكتبات المطلوبة
  ```toml
  [dependencies]
  tauri = "2.0"
  windows = { version = "0.58", features = ["Win32_*"] }
  rusqlite = { version = "0.32", features = ["fts5"] }
  tokio = { version = "1", features = ["full"] }
  serde = { version = "1", features = ["derive"] }
  ```

- [ ] 2.2: تنفيذ `WS_EX_NOACTIVATE` (حل مشكلة Focus Stealing)
  
  **الملف:** `src-tauri/src/window/noactivate.rs`
  
  **الوظيفة:**
  - Subclass Window Procedure
  - اعتراض رسالة `WM_MOUSEACTIVATE`
  - إرجاع `MA_NOACTIVATE`
  - Fallback: حفظ واستعادة النافذة النشطة

- [ ] 2.3: تنفيذ Clipboard Monitor
  
  **الملف:** `src-tauri/src/input/clipboard.rs`
  
  **الوظيفة:**
  - مراقبة `WM_CLIPBOARDUPDATE`
  - قراءة محتوى الحافظة
  - إرسال للـ Frontend عبر Tauri Events

- [ ] 2.4: تنفيذ Global Keyboard Injection
  
  **الملف:** `src-tauri/src/input/keyboard.rs`
  
  **الوظيفة:**
  - دالة `send_copy()` - ترسل Ctrl+C
  - دالة `send_paste()` - ترسل Ctrl+V
  - استخدام `SendInput` Windows API

- [ ] 2.5: تنفيذ SQLite Database Layer
  
  **الملف:** `src-tauri/src/db/schema.rs`
  
  ```sql
  CREATE TABLE clipboard_entries (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      content TEXT NOT NULL,
      content_type TEXT CHECK(content_type IN ('text', 'code', 'markdown')),
      created_at INTEGER NOT NULL,
      is_pinned INTEGER DEFAULT 0,
      is_scrubbed INTEGER DEFAULT 0
  );
  
  CREATE VIRTUAL TABLE clipboard_fts USING fts5(
      content,
      content = 'clipboard_entries',
      tokenize = 'unicode61 remove_diacritics 2'
  );
  ```

- [ ] 2.6: تنفيذ FTS5 Search Queries
  
  **الملف:** `src-tauri/src/db/queries.rs`
  
  **الوظيفة:**
  - `search_history(query: &str)` - بحث في السجل
  - `get_recent(limit: u32)` - أحدث العناصر
  - `pin_item(id: i64)` - تثبيت عنصر

- [ ] 2.7: تنفيذ Tauri Commands
  
  **Commands:**
  ```rust
  #[tauri::command]
  async fn send_copy() -> Result<(), String>
  
  #[tauri::command]
  async fn send_paste() -> Result<(), String>
  
  #[tauri::command]
  async fn search_clipboard(query: String) -> Result<Vec<ClipboardEntry>, String>
  
  #[tauri::command]
  async fn pin_entry(id: i64) -> Result<(), String>
  ```

**✅ مكتملة بواسطة:** [سيُملأ بعد التنفيذ]  
**📅 التاريخ:** [سيُملأ بعد التنفيذ]

---

### المرحلة 3: بناء Design System - CSS Foundation

**النموذج المناسب:** Claude Sonnet 4.5 (Thinking)

**السبب من SOTA_Models_2026:**
- متوازن بين الجودة والسرعة
- ممتاز في Creative Writing والتفاصيل الدقيقة
- أقل تكلفة من Opus مع جودة عالية للـ CSS

#### المهام:

- [ ] 3.1: إنشاء Tailwind Config مع ألوان Manus
  
  **الملف:** `tailwind.config.js`
  
  ```javascript
  module.exports = {
    theme: {
      extend: {
        colors: {
          'base': '#1C1D21',
          'surface': '#2E3440',
          'hover': '#3B4252',
          'border': '#3E4451',
          'accent': '#88C0D0',
          'text': {
            primary: '#E5E9F0',
            secondary: '#D8DEE9',
            muted: '#4C566A',
          }
        },
        fontFamily: {
          'latin': ['Geist Sans', 'Inter', 'system-ui'],
          'arabic': ['IBM Plex Sans Arabic', 'Segoe UI'],
          'mono': ['Geist Mono', 'IBM Plex Mono', 'monospace'],
        },
      }
    }
  }
  ```

- [ ] 3.2: تنزيل وتضمين الخطوط
  - Geist Sans Variable
  - Geist Mono Variable
  - IBM Plex Sans Arabic (Regular, Medium, Bold)
  
  **المجلد:** `public/fonts/`

- [ ] 3.3: إنشاء Global CSS مع RTL Support
  
  **الملف:** `src/index.css`
  
  ```css
  @layer base {
    code {
      direction: ltr;
      unicode-bidi: isolate;
      background: rgba(255, 255, 255, 0.08);
      border: 1px solid rgba(255, 255, 255, 0.1);
      border-radius: 4px;
      padding-inline: 4px;
    }
    
    [dir="rtl"] * {
      font-family: var(--font-arabic), var(--font-latin);
    }
  }
  ```

- [ ] 3.4: إنشاء CSS Logical Properties Utilities
  - `margin-inline-start`, `margin-inline-end`
  - `padding-inline-start`, `padding-inline-end`
  - `inset-inline-start`, `inset-inline-end`

**✅ مكتملة بواسطة:** [سيُملأ بعد التنفيذ]  
**📅 التاريخ:** [سيُملأ بعد التنفيذ]

---

### المرحلة 4: بناء UI Components - Frontend

**النموذج المناسب:** Claude Sonnet 4.5

**السبب من SOTA_Models_2026:**
- جودة كود React ممتازة
- أسرع من Opus (مهم للتطوير السريع)
- كافي لـ Component-level code

#### المهام:

- [ ] 4.1: إنشاء Zustand Store للـ State Management
  
  **الملف:** `src/stores/clipboardStore.ts`
  
  ```typescript
  interface ClipboardStore {
    entries: ClipboardEntry[];
    addEntry: (entry: ClipboardEntry) => void;
    searchEntries: (query: string) => Promise<void>;
    pinEntry: (id: number) => void;
  }
  ```

- [ ] 4.2: تنفيذ Dock Component
  
  **الملف:** `src/components/Dock/Dock.tsx`
  
  **الميزات:**
  - قائمة رأسية للعناصر
  - أيقونات Type (Code / Text / MD)
  - Timestamp labels
  - Hover trigger للـ Bubble

- [ ] 4.3: تنفيذ Hover Bubble Component
  
  **الملف:** `src/components/Bubble/HoverBubble.tsx`
  
  **الميزات:**
  - يظهر بعد 200ms hover
  - موضع ديناميكي (عكس الـ Dock)
  - Width: auto حتى 400px
  - Backdrop blur effect
  - Markdown rendering

- [ ] 4.4: تنفيذ Markdown Editor
  
  **الملف:** `src/components/Editor/MarkdownEditor.tsx`
  
  **الميزات:**
  - Edit Mode (textarea)
  - Preview Mode (rendered HTML)
  - Toggle button
  - "Load from Clipboard" button
  - RTL support

- [ ] 4.5: تنفيذ Markdown Rendering Pipeline
  
  **الملف:** `src/utils/markdown.ts`
  
  ```typescript
  import { unified } from 'unified';
  import remarkParse from 'remark-parse';
  import remarkRehype from 'remark-rehype';
  import rehypeStringify from 'rehype-stringify';
  import { getHighlighter } from 'shiki';
  
  async function renderMarkdown(content: string): Promise<string> {
    // unified pipeline + Shiki syntax highlighting
  }
  ```

- [ ] 4.6: تنفيذ RTL Detection Utility
  
  **الملف:** `src/utils/bidi.ts`
  
  ```typescript
  function detectDirection(text: string): 'ltr' | 'rtl' {
    const rtlRegex = /[\u0591-\u07FF\uFB1D-\uFDFD\uFE70-\uFEFC]/;
    return rtlRegex.test(text) ? 'rtl' : 'ltr';
  }
  ```

- [ ] 4.7: تنفيذ Smart Scrub Feature
  
  **الملف:** `src/utils/smartScrub.ts`
  
  **الوظيفة:**
  - اكتشاف مقدمات AI (Regex patterns)
  - حذف النص الزائد
  - استخراج Code blocks
  - إظهار Undo Toast

- [ ] 4.8: تنفيذ Sparkle Animation
  
  **CSS:**
  ```css
  @keyframes sparkle {
    0%, 100% { transform: scale(1) rotate(0deg); }
    50% { transform: scale(1.2) rotate(180deg); }
  }
  
  .sparkle-icon {
    animation: sparkle 2s ease-in-out infinite;
    filter: drop-shadow(0 0 8px #FFD700);
  }
  ```

- [ ] 4.9: تنفيذ Focus State Visual Feedback
  - Passive State: 85% opacity
  - Active State: 100% opacity + Cyan glow
  - Transition: 200ms ease-out

**✅ مكتملة بواسطة:** [سيُملأ بعد التنفيذ]  
**📅 التاريخ:** [سيُملأ بعد التنفيذ]

---

### المرحلة 5: Window Management & Docking

**النموذج المناسب:** Claude Opus 4.5 (Thinking)

**السبب من SOTA_Models_2026:**
- نرجع لـ Opus لأن Multi-Monitor معقد جداً
- يحتاج System-level thinking
- الأمان مهم (Window positioning قد يسبب crashes)

#### المهام:

- [ ] 5.1: إنشاء Window Configuration في Tauri
  
  **الملف:** `src-tauri/tauri.conf.json`
  
  ```json
  {
    "windows": [{
      "title": "Floating MD",
      "width": 80,
      "height": 600,
      "decorations": false,
      "transparent": true,
      "alwaysOnTop": true,
      "skipTaskbar": true,
      "resizable": false
    }]
  }
  ```

- [ ] 5.2: تنفيذ Docking Logic (Rust)
  
  **الملف:** `src-tauri/src/window/docking.rs`
  
  **الوظيفة:**
  - اكتشاف حواف الشاشة
  - Snap to edge عند السحب
  - حفظ Position في Database
  - استعادة Position عند البدء

- [ ] 5.3: تنفيذ Auto-Hide/Peeking
  
  **الملف:** `src/components/Dock/DockBehavior.tsx`
  
  **الوظيفة:**
  - تقليص إلى "لسان" عند عدم الاستخدام
  - توسيع عند Hover
  - Collapse timer (3 ثوان)

- [ ] 5.4: Multi-Monitor Support
  
  **الوظيفة:**
  - حفظ Monitor ID مع Position
  - اكتشاف Monitor changes
  - التعامل مع Monitor disconnect

- [ ] 5.5: Global Hotkey للإظهار/الإخفاء
  
  **الملف:** `src-tauri/src/hotkey.rs`
  
  **Hotkey:** `Ctrl+Shift+Space` (قابل للتخصيص)

**✅ مكتملة بواسطة:** [سيُملأ بعد التنفيذ]  
**📅 التاريخ:** [سيُملأ بعد التنفيذ]

---

### المرحلة 6: Testing & Optimization

**النموذج المناسب:** Claude Sonnet 4.5

**السبب من SOTA_Models_2026:**
- منهجي في الاختبار
- يكتب test cases شاملة
- كافي لمرحلة الـ QA

#### المهام:

- [ ] 6.1: اختبار Focus Stealing على Windows 10/11
  - فتح Word
  - ضغط Paste من Floating MD
  - التأكد من عدم فقدان Focus

- [ ] 6.2: اختبار RTL/BiDi
  - نسخ نص عربي مع كود إنجليزي
  - التأكد من العرض الصحيح
  - اختبار في Editor Mode

- [ ] 6.3: اختبار Smart Scrub
  - نسخ رد من ChatGPT مع مقدمة
  - ضغط Sparkle
  - التأكد من حذف المقدمة فقط
  - اختبار Undo

- [ ] 6.4: اختبار Multi-Monitor
  - نقل النافذة بين شاشتين
  - إعادة التشغيل
  - التأكد من استعادة الموضع

- [ ] 6.5: اختبار FTS5 Search
  - إضافة 100+ عنصر
  - بحث بالعربية
  - بحث بالإنجليزية
  - قياس السرعة (يجب أقل من 10ms)

- [ ] 6.6: قياس الأداء
  - Memory usage (Idle)
  - Startup time
  - Bundle size
  - مقارنة مع الأهداف

- [ ] 6.7: Optimization
  - Code splitting (Lazy load Editor)
  - Tree shaking
  - Font subsetting

**✅ مكتملة بواسطة:** [سيُملأ بعد التنفيذ]  
**📅 التاريخ:** [سيُملأ بعد التنفيذ]

---

### المرحلة 7: Documentation & Build

**النموذج المناسب:** Gemini 3 Flash

**السبب من SOTA_Models_2026:**
- الأسرع والأرخص (0.50 دولار/M)
- كافي جداً للتوثيق
- مهام بسيطة لا تحتاج Opus

#### المهام:

- [ ] 7.1: كتابة README.md
  - وصف المشروع
  - تعليمات التثبيت
  - لقطات شاشة
  - Keyboard shortcuts

- [ ] 7.2: كتابة ARCHITECTURE.md
  - شرح المعمارية
  - Data flow diagrams
  - API Reference

- [ ] 7.3: إنشاء Build للإنتاج
  ```bash
  pnpm tauri build
  ```

- [ ] 7.4: إنشاء Installer
  - MSI installer لـ Windows
  - تضمين WebView2 Runtime

- [ ] 7.5: إنشاء نسخة احتياطية
  ```
  Backups/v1.0.0_20260123_HHMMSS/
  ├── Source/
  └── Installer/
  ```

- [ ] 7.6: Git Tagging
  ```bash
  git tag -a v1.0.0 -m "First stable release"
  git push origin v1.0.0
  ```

**✅ مكتملة بواسطة:** [سيُملأ بعد التنفيذ]  
**📅 التاريخ:** [سيُملأ بعد التنفيذ]

---

## 🔬 Verification Plan

### Automated Tests

#### 1. Backend Tests (Rust)
```bash
cd src-tauri
cargo test
```

**الاختبارات المطلوبة:**
- `test_clipboard_monitor()` - مراقبة الحافظة
- `test_send_input()` - إرسال Ctrl+C/V
- `test_fts5_search()` - البحث العربي
- `test_noactivate_wndproc()` - Focus handling

#### 2. Frontend Tests (Vitest)
```bash
pnpm test
```

**الاختبارات المطلوبة:**
- `test_markdown_rendering()` - تحويل MD إلى HTML
- `test_rtl_detection()` - كشف اتجاه النص
- `test_smart_scrub()` - تنظيف AI text
- `test_bidi_isolation()` - عزل الأكواد

### Manual Verification

#### الاختبار 1: Focus Stealing
1. فتح Microsoft Word
2. كتابة نص في Word
3. نسخ شيء في Floating MD
4. ضغط زر "Paste" في Floating MD
5. ✅ **النتيجة المتوقعة:** النص يُلصق في Word بدون فقدان التركيز

#### الاختبار 2: RTL Text
1. نسخ هذا النص:
   ```
   الدالة print("hello") تطبع النص على الشاشة
   ```
2. فتح Markdown Editor
3. ✅ **النتيجة المتوقعة:** الكود يظهر LTR داخل النص العربي RTL

#### الاختبار 3: Smart Scrub
1. نسخ رد من ChatGPT:
   ```
   Certainly! Here is the Python code you requested:
   
   ```python
   print("Hello World")
   ```
   
   I hope this helps!
   ```
2. ضغط Sparkle ✨
3. ✅ **النتيجة المتوقعة:** يبقى فقط:
   ```python
   print("Hello World")
   ```

#### الاختبار 4: Multi-Monitor
1. توصيل شاشة ثانية
2. سحب Floating MD للشاشة الثانية
3. وضعه على الحافة اليمنى
4. إعادة تشغيل البرنامج
5. ✅ **النتيجة المتوقعة:** يظهر في نفس المكان (الشاشة الثانية، حافة يمنى)

#### الاختبار 5: Performance
1. فتح Task Manager
2. قياس Memory usage لـ Floating MD
3. ✅ **النتيجة المتوقعة:** أقل من 80 MB RAM

---

## 📊 Timeline المتوقع

| المرحلة | التعقيد | الوقت المقدر |
|---------|---------|--------------|
| **المرحلة 1** | 🟢 منخفض | 30 دقيقة |
| **المرحلة 2** | 🔴 عالي جداً | 4-6 ساعات |
| **المرحلة 3** | 🟡 متوسط | 1-2 ساعة |
| **المرحلة 4** | 🟠 عالي | 3-4 ساعات |
| **المرحلة 5** | 🔴 عالي جداً | 3-4 ساعات |
| **المرحلة 6** | 🟠 عالي | 2-3 ساعات |
| **المرحلة 7** | 🟢 منخفض | 1 ساعة |

**الإجمالي:** 14-20 ساعة عمل فعلي

---

## 📌 ملاحظات مهمة

### ⚠️ التحديات التقنية المتوقعة

1. **WS_EX_NOACTIVATE قد يفشل** على بعض إصدارات Windows
   - **الحل:** Fallback لحفظ واستعادة النافذة النشطة

2. **WebView2 قد لا يكون مثبت** على بعض الأجهزة
   - **الحل:** تضمين WebView2 Bootstrapper في Installer

3. **Font Loading قد يكون بطيء**
   - **الحل:** استخدام `font-display: swap` و Font subsetting

4. **Multi-Monitor handling** معقد على Windows
   - **الحل:** حفظ Monitor ID مع الموضع النسبي

---

## 🎯 معايير النجاح

- [x] التطبيق يبدأ في أقل من 500ms
- [x] استهلاك الذاكرة أقل من 80 MB
- [x] حجم Installer أقل من 15 MB
- [x] Copy/Paste يعمل بدون Focus Stealing
- [x] البحث في 10,000 عنصر يأخذ أقل من 10ms
- [x] النص العربي يظهر بشكل صحيح مع الأكواد الإنجليزية
- [x] Smart Scrub يحذف المقدمات بدقة 95%+

---

## 📚 المراجع

- [Tauri 2.0 Documentation](https://v2.tauri.app/)
- [windows-rs Documentation](https://microsoft.github.io/windows-docs-rs/)
- [unified Documentation](https://unifiedjs.com/)
- [Shiki Documentation](https://shiki.style/)
- [W3C BiDi Guidelines](https://www.w3.org/International/articles/inline-bidi-markup/)

---

**آخر تحديث:** 2026-01-23 00:52  
**الحالة:** **جاهزة للمراجعة** ✋
