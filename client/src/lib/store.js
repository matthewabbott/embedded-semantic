import { writable } from 'svelte/store';

export const collection = writable(null);
export const documentCount = writable(0);
export const uploadStatus = writable({
    isUploading: false,
    message: '',
    documentsAdded: 0
});