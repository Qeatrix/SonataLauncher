import { Reactive } from "hywer/jsx-runtime";
import { gsap } from 'gsap/all';
import { getLastProcessedElement, getWorkNames } from "./element";

export const startElapseTimer = (taskElapsedTime: Reactive<string>) => {
    const timerStartTime = new Date();

    const timerInterval = setInterval(() => {
        const elapsedSeconds = new Date().getTime() - timerStartTime.getTime();
        const seconds = (elapsedSeconds / 1000).toFixed(1);
        taskElapsedTime.val = `${seconds}s`;
    }, 100)

    return timerInterval;
}

export const stopElapseTimer = (timerInterval: number, taskElapsedTime: Reactive<string>) => {
    clearInterval(timerInterval);
    taskElapsedTime.val = "0.0s";
}

export const startWorknameBlinkTimer = (
    componentId: string,
    currentWorkId: Reactive<number>,
    css: any,
    worknamesBlinked: boolean
) => {
    const transitionType = "power4.Out"
    const transitionDuration = 1;

    // return setInterval(() => {
    //     if (workName) {
    //         gsap.to(workName[0], {
    //             opacity: worknamesBlinked ? 1 : 0,
    //             ease: transitionType,
    //             duration: transitionDuration,
    //         })

    //         gsap.to(workName[1], {
    //             opacity: worknamesBlinked ? 0 : 1,
    //             ease: transitionType,
    //             duration: transitionDuration,
    //         })

    //         if (worknamesBlinked) {
    //             worknamesBlinked = false;
    //         } else {
    //             worknamesBlinked = true;
    //         }
    //     } else {
    //         console.warn("Workname not found");
    //     }
    // }, 4000)

    setTimeout(() => {
        const workNames = getWorkNames(componentId, currentWorkId.val);

        if (workNames) {
            gsap.to(workNames[0], {
                opacity: 0,
                ease: transitionType,
                duration: transitionDuration,
                onComplete: () => {
                    console.log(workNames[0]);
                }
            })

            gsap.to(workNames[1], {
                opacity: 1,
                ease: transitionType,
                duration: transitionDuration,
                onComplete: () => {
                    console.log(workNames[1]);
                }
            })
        }
    }, 3000);
}

export const stopWorknameBlinkTimer = (timerInterval: number, worknamesBlinked: boolean) => {
    clearInterval(timerInterval);
    worknamesBlinked = false;
}
