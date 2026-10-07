function resetInput() {
    const sentinel = "\u200b\u200b";
    const ta = document.querySelector(".engine-textarea");
    if (!ta) return false;
    if (ta.value !== sentinel) ta.value = sentinel;
    ta.setSelectionRange(1,1);
    return true;
}
