export async function copyToClipboard(text: string) {
    await navigator.clipboard.writeText(text);
    const checkIcon = document.getElementById('check-icon');
    if (checkIcon) {
        checkIcon.classList.remove('invisible');
        setTimeout(() => checkIcon.classList.add('invisible'), 2000);
    }
}
