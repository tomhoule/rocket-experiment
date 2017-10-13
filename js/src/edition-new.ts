
// type Predicate<T> = (input: T) => boolean;

// function map<T, U>(opt: T | null, mapper: (inner: T) => U): U | null {
//     if (opt === null) {
//         return null
//     } else {
//         return mapper(opt)
//     }
// }

// function ensureClass(elem: HTMLElement, className: string) {
//     if (!elem.classList.contains(className)) {
//         elem.classList.add(className)
//     }
// }

// function precludeClass(elem: HTMLElement, className: string) {
//     if (elem.classList.contains(className)) {
//         elem.classList.remove(className)
//     }
// }

// function iconContainer(): HTMLSpanElement {
//     const elem = document.createElement('span')
//     elem.classList.add('icon')
//     elem.classList.add('is-small')
//     elem.classList.add('is-right')
//     return elem
// }

// function successIcon(): HTMLSpanElement {
//     const container = iconContainer()
//     const icon = document.createElement('i')
//     icon.classList.add('fa')
//     icon.classList.add('fa-warning')
//     container.appendChild(icon)
//     return container
// }

// function validateWith(elem: HTMLInputElement, validator: Predicate<string>) {
//     elem.addEventListener('change', function(event) {
//         if (validator(this.value)) {
//             ensureClass(this, 'is-success')
//             precludeClass(this, 'is-danger')
//             map(this.parentElement.querySelector('span'), elem => elem.remove())
//             this.parentElement.appendChild(successIcon())
//         } else {
//             ensureClass(this, 'is-danger')
//             precludeClass(this, 'is-success')
//         }
//     })
// }

// function validate(fieldName: string, validator: Predicate<string>) {
//     const element = document.querySelector(`[name=${fieldName}`) as HTMLInputElement | null
//     map(element, elem => validateWith(elem, validator))
// }

// function required(input: string): boolean {
//     return !(input.length === 0)
// }

// document.addEventListener('DOMContentLoaded', () => {
//     validate('title', required)
// })
