// src/app.d.ts

/// <reference types="@sveltejs/kit" />
/// <reference types="unplugin-icons/types/svelte" />

import type { Session } from '@auth/core/types';

declare global {
	namespace App {
		interface Locals {
			getSession(): Promise<Session>;
		}
		//interface PageData {}
		interface Error {
			code: number;
			message: string;
			id?: string;
			problem?: {
				status: number;
				title: string;
				description?: string;
			};
		}
		// interface Platform {}
	}
}
