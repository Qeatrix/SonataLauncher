import { Reactive } from "hywer/jsx-runtime";

export const getLastProcessedElement = (componentId: string, currentWorkId: Reactive<number>) => {
    return document.getElementById(`ProgressItem-${componentId}-${currentWorkId.val}`);
}

export const getWorkNames = (componentId: string, currentWorkId: number) => {
    const firstWorkName = document.getElementById(`ProgressItemName-${componentId}-${currentWorkId}-${0}`);
    const secondWorkName = document.getElementById(`ProgressItemName-${componentId}-${currentWorkId}-${1}`);

    if (firstWorkName && secondWorkName) {
        return [firstWorkName, secondWorkName];
    } else {
        return null;
    }
}


export const constructElementId = (componentId: string, i: number) => {
    return `ProgressItem-${componentId}-${i}`
}

export const constructElementNameId = (componentId: string, i: number, subi: number) => {

    return `ProgressItemName-${componentId}-${i}-${subi}`
}

export const constructElementIconId = (componentId: string, i: number) => {
    return `ProgressItemIcon-${componentId}-${i}`
}
