import { Translation } from "../types";
import defaultTranslation from '@/data/localization/en.json';

class TranslationStore {
    private static translationsKey = 'app-translations';
    private static languageKey = 'app-lang';
    private translation: Record<string, any> = defaultTranslation;
    private language: string = 'en';

    constructor() {
        this.loadTranslation();
    }

    private loadTranslation() {
        const storedTranslation = localStorage.getItem(TranslationStore.translationsKey);

        if (storedTranslation) {
            // this.translation = JSON.parse(defaultTranslation);
        } else {
            this.translation = defaultTranslation;
            localStorage.setItem(TranslationStore.translationsKey, JSON.stringify(this.translation));
        }
    }

    public getTranslation(): Translation {
        return this.translation;
    }

    public setTranslation(translation: Translation) {
        this.translation = translation;
        localStorage.setItem(TranslationStore.translationsKey, JSON.stringify(translation));
    }

    public t(key: string): string {
        const keys = key.split('.');
        let translation = this.translation;

        for (const k of keys) {
            if (translation[k]) {
                translation = translation[k];
            } else {
                return key; // Fallback to key if translation is not found
            }
        }

        return translation;
    }

}

const TranslationStoreInstance = new TranslationStore();
export default TranslationStoreInstance;