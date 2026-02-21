# 🐛 تقرير تشخيص المشكلة الحرجة (Bug Report)

> **التاريخ:** 2026-02-06  
> **المشكلة:** جميع أزرار الشريط لا تستجيب (ما عدا Markdown)

---

## 🔍 السبب الجذري (Root Cause)

### المشكلة:
في `src/components/Toolbar/Toolbar.tsx` السطر **93**:

```tsx
<div
    data-tauri-drag-region  // ❌ هنا المشكلة!
    onMouseEnter={show}
    onMouseLeave={startHideTimer}
    className={`...`}
>
    {/* الأزرار هنا */}
</div>
```

**الشرح:**
- `data-tauri-drag-region` يخلي المنطقة كلها "drag area"
- Tauri بيعترض **جميع** mouse events في هذه المنطقة
- الـ clicks **لا تصل** للأزرار داخل الـ div
- زر Markdown يعمل لأنه له **special handling** في `handleItemClick`

---

## ✅ الحل

### الخيار 1: إزالة drag-region من الـ container (الأسهل)
```tsx
<div
    // ❌ إزالة: data-tauri-drag-region
    onMouseEnter={show}
    onMouseLeave={startHideTimer}
    className={`...`}
>
```

**المشكلة:** لن يمكن سحب النافذة من أي مكان!

---

### الخيار 2: إضافة spacer منفصل للـ drag (الأفضل)
```tsx
<div className="relative">
    {/* Drag Handle - منطقة فارغة للسحب */}
    <div 
        data-tauri-drag-region
        className="absolute inset-0 pointer-events-none"
        style={{ zIndex: -1 }}
    />
    
    {/* الأزرار - بدون drag region */}
    <div
        onMouseEnter={show}
        onMouseLeave={startHideTimer}
        className={`...`}
    >
        {/* الأزرار هنا */}
    </div>
</div>
```

**المشكلة:** معقد شوية.

---

### الخيار 3: استثناء الأزرار من drag-region (الموصى به)
```tsx
<div
    data-tauri-drag-region
    onMouseEnter={show}
    onMouseLeave={startHideTimer}
    className={`...`}
>
    {/* الأزرار */}
    <div 
        data-tauri-drag-region="false"  // ✅ استثناء الأزرار
        className="flex items-center gap-2"
    >
        {items.map(...)}
    </div>
</div>
```

**لكن:** Tauri **لا يدعم** `data-tauri-drag-region="false"`!

---

### ✅ الحل النهائي (الأبسط والأفضل):

نقل `data-tauri-drag-region` لعنصر **padding** فقط، ونستثني الأزرار:

```tsx
<div className="relative">
    {/* Main container - بدون drag */}
    <div
        onMouseEnter={show}
        onMouseLeave={startHideTimer}
        className={`flex items-center gap-3 p-3 ${containerClasses}`}
    >
        {/* Drag handle - padding area only */}
        <div 
            data-tauri-drag-region
            className="absolute inset-0 -z-10"
        />
        
        {/* Buttons container */}
        <div className="relative z-10 flex items-center gap-2">
            {items.map(...)}
        </div>
    </div>
</div>
```

---

## 🎯 الخطة

1. تعديل `Toolbar.tsx` حسب الحل النهائي
2. اختبار السحب (يجب أن يعمل)
3. اختبار الأزرار (يجب أن تعمل)
4. إصلاح الحركة غير السلسة (مشكلة منفصلة)

---

**الوقت المقدر:** 15 دقيقة
