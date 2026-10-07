function hitTest(x, y) {
    let node = null, offset = 0;
    if (document.caretPositionFromPoint) {
        const p = document.caretPositionFromPoint(x, y);
        if (p) { node = p.offsetNode; offset = p.offset; }
    } else if (document.caretRangeFromPoint) {
        const r = document.caretRangeFromPoint(x, y);
        if (r) { node = r.startContainer; offset = r.startOffset; }
    }
    if (!node) return null;

    const el = node.nodeType === Node.TEXT_NODE ? node.parentElement : node;
    const target = el.closest('[data-start]');
    const model = document.elementFromPoint(x, y)?.closest('.engine-model');
    if (!target || !model || !model.contains(target)) return null;

    const start = Number(target.dataset.start);

    // clicked the line itself: an empty line, or past the end of the text
    if (target.classList.contains('engine-line')) {
        return offset > 0 ? Number(target.dataset.end) : start;
    }
    // the virtual end-of-line selection block has no real text
    if (target.classList.contains('eol') || node.nodeType !== Node.TEXT_NODE) {
        return start;
    }
    // token span: its start plus the UTF-8 length of the text before the caret
    const before = node.textContent.slice(0, offset);
    return start + new TextEncoder().encode(before).length;
}