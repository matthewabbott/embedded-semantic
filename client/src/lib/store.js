import { writable } from 'svelte/store';

export const collection = writable(null);
export const neuralCollection = writable(null);
export const searchMode = writable('tf-idf');
export const documentCount = writable(0);
export const uploadStatus = writable({
    isUploading: false,
    message: '',
    documentsAdded: 0
});