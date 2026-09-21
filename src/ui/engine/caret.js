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
    const line = el.closest('[data-line]');
    const clicked = document.elementFromPoint(x, y)?.closest('.engine-model');
    if (!line || !clicked || !clicked.contains(line)) return null;

    let col;
    if (node.nodeType === Node.TEXT_NODE) {
        col = Array.from(node.textContent.slice(0, offset)).length;
    } else {
        col = offset > 0 ? Array.from(line.textContent).length : 0;
    }
    return [Number(line.dataset.line), col];
}