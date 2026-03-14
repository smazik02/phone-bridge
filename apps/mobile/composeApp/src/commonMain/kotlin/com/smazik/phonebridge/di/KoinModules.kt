package com.smazik.phonebridge.di

import org.koin.core.module.Module
import org.koin.dsl.module

val sharedModule = module {}

expect val platformModule: Module