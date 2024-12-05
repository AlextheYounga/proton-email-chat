<template>
	<header class="bg-slate-700 shadow">
		<div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
			<h1 class="text-3xl font-bold tracking-tight text-white">Importer</h1>
		</div>
	</header>
	<div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
		<form @submit.prevent="submitForm">
			<div class="space-y-12">
				<div class="border-b border-white/10 pb-12">
					<h2 class="text-base/7 font-semibold text-white">Import Emails</h2>
					<p class="mt-1 text-sm/6 text-gray-200">This information will be displayed publicly so be careful
						what you share.</p>
					<div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
						<div class="col-span-full">
							<label for="cover-photo" class="block text-sm/6 font-medium text-white">Cover photo</label>
							<div
								class="mt-2 flex justify-center rounded-lg border border-dashed border-white/25 px-6 py-10">
								<div class="text-center">
									<PhotoIcon class="mx-auto size-12 text-gray-500" aria-hidden="true" />
									<div v-if="!filesReady" class="mt-4 flex text-sm/6 text-gray-400">
										<label for="file-upload"
											class="relative cursor-pointer rounded-md bg-gray-900 font-semibold text-white focus-within:outline-none focus-within:ring-2 focus-within:ring-indigo-600 focus-within:ring-offset-2 focus-within:ring-offset-gray-900 hover:text-indigo-500">
											<span>Upload a folder</span>
											<input id="file-upload" name="file-upload" type="file" class="sr-only"
												webkitdirectory @change="handleFolderUpload" />
										</label>
										<p class="pl-1">or drag and drop</p>
									</div>
									<p class="text-xs/5 text-gray-400">PNG, JPG, GIF up to 10MB</p>
									<div v-if="isLoading" class="mt-4 text-sm text-gray-400">Loading files...</div>
									<div v-if="filesReady" class="mt-4 text-sm text-green-400">Files are ready to be imported.</div>
									<div v-if="isImporting" class="mt-4 w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
										<div class="bg-blue-600 h-2.5 rounded-full" :style="{ width: progress + '%' }"></div>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>

			<div class="mt-6 flex items-center justify-end gap-x-6">
				<button type="button" class="text-sm/6 font-semibold text-white">Cancel</button>
				<button type="submit"
					class="rounded-md bg-indigo-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500">Upload</button>
			</div>
		</form>
	</div>
</template>

<script setup>
import { ref } from 'vue'
import { PhotoIcon } from '@heroicons/vue/24/solid'
import { importEmail } from './js/email-importer';

const files = ref([])
const isLoading = ref(false)
const filesReady = ref(false)
const isImporting = ref(false)
const progress = ref(0)

// Handle folder input change
const handleFolderUpload = (event) => {
	isLoading.value = true
	filesReady.value = false
	files.value = Array.from(event.target.files) // Convert FileList to Array
	isLoading.value = false
	filesReady.value = true
}

// Handle form submission
const submitForm = async () => {
	if (!files.value.length) {
		alert('Please select a folder before saving.')
		return
	}

	isImporting.value = true
	progress.value = 0
	let processedFiles = 0

	const emailFiles = files.value.filter(file => file.name.endsWith(".eml"))
	const metadataFiles = files.value.filter(file => file.name.endsWith(".json"))

	for (const file of emailFiles) {
			const protonId = file.name.split(".")[0];
			const metadataFile = metadataFiles.find(metadataFile => metadataFile.name === `${protonId}.metadata.json`)
			await importEmail(file, metadataFile)

		processedFiles++
		progress.value = (processedFiles / emailFiles.length) * 100
	}
}
</script>