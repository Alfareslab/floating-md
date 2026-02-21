# 📁 ملفات الخطوط

بسبب حقوق الترخيص وحجم الملفات، الخطوط تحتاج تنزيل يدوي.

---

## 🔤 الخطوط المطلوبة:

### 1. Geist Sans Variable (للنصوص اللاتينية)
- **المصدر:** https://vercel.com/font
- **اسم الملف:** `GeistSans-Variable.woff2`
- **الترخيص:** SIL Open Font License (مجاني)

### 2. Geist Mono Variable (للأكواد البرمجية)
- **المصدر:** https://vercel.com/font
- **اسم الملف:** `GeistMono-Variable.woff2`
- **الترخيص:** SIL Open Font License (مجاني)

### 3. IBM Plex Sans Arabic (للنصوص العربية)
- **المصدر:** https://fonts.google.com/specimen/IBM+Plex+Sans+Arabic
- **اسم الملف:** `IBMPlexSansArabic-Regular.woff2`
- **الترخيص:** SIL Open Font License (مجاني)

---

## 📥 خطوات التثبيت:

### الخطوة 1: تنزيل خطوط Geist
1. افتح الرابط: https://vercel.com/font
2. اضغط على زر "Download" لتنزيل الحزمة
3. فك الضغط عن الملف
4. ابحث عن الملفات:
   - `GeistSans-Variable.woff2`
   - `GeistMono-Variable.woff2`

### الخطوة 2: تنزيل خط IBM Plex Arabic
1. افتح الرابط: https://fonts.google.com/specimen/IBM+Plex+Sans+Arabic
2. اضغط على "Download family" في أعلى اليمين
3. فك الضغط عن الملف
4. الملف اللي تحتاجه: `IBMPlexSansArabic-Regular.woff2`
   - (لو مش موجود بصيغة woff2، استخدم محول أونلاين)

### الخطوة 3: وضع الملفات
1. انسخ الملفات الـ 3 إلى هذا المجلد:
   ```
   public/fonts/
   ```

2. تأكد إن أسماء الملفات بالظبط كالتالي:
   - `GeistSans-Variable.woff2`
   - `GeistMono-Variable.woff2`
   - `IBMPlexSansArabic-Regular.woff2`

### الخطوة 4: تأكيد التثبيت
الخطوط هتتحمل تلقائياً من خلال ملف `src/styles.css`

---

## 🔄 الخطوط الاحتياطية:
لو الخطوط المخصصة مش موجودة، النظام هيستخدم بدائل:
- **للإنجليزي:** Inter, system-ui, sans-serif
- **للعربي:** Segoe UI, sans-serif
- **للأكواد:** Consolas, monospace

---

## ⚡ ملاحظة سريعة:
لو مش عايز تنزل الخطوط دلوقتي، البرنامج هيشتغل عادي بالخطوط الاحتياطية.
الخطوط المخصصة بتدي شكل أحلى بس مش إلزامية للتشغيل.
