<script setup lang="ts">
import {FormControl, FormField, FormItem, FormLabel, FormMessage} from '@/components/ui/form'

import {toTypedSchema} from "@vee-validate/zod";
import * as z from "zod";
import {useForm} from "vee-validate";
import {Input} from "@/components/ui/input";
import {Button} from "@/components/ui/button";
import {Icon} from "@iconify/vue";
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {listen} from '@tauri-apps/api/event'
import {useRouter} from "vue-router";
import {MqttConnectionEvent, useConnectionStore} from "@/store/connection.store.ts";

enum ConnectionCheck {
  IDLE,
  IN_PROGRESS,
  SUCCESS,
  ERROR,
}

const connectionCheck = ref<ConnectionCheck>(ConnectionCheck.IDLE)

const router = useRouter();
const formSchema = toTypedSchema(z.object({
  name: z.string().refine(name => !connectionStore.connections.has(name), {
    error: 'Connection with this name already exists',
  }),
  mqttUrl: z.url({protocol: /^(mqtt|mqtts)/, error: 'Invalid MQTT URL'}).nonempty().nonoptional(),
  apiUrl: z.url({protocol: /^(http|https)/, error: 'Invalid API URL'}).nonempty().nonoptional(),
  username: z.string(),
  password: z.string(),
}));

const form = useForm({
  validationSchema: formSchema,
  initialValues: {
    name: 'Labeq Gateway',
    mqttUrl: '',
    apiUrl: '',
    username: '',
    password: '',
  }
})

const connectionStore = useConnectionStore()
const onSubmit = form.handleSubmit(values => {
  connectionStore.connections.set(values.name, {
    connected: false,
    id: values.name,
    mqttUrl: values.mqttUrl,
    apiUrl: values.apiUrl,
    username: values.username,
    password: values.password,
    devices: []
  })
  invoke('gateway_connect', {
    url: values.mqttUrl,
    apiUrl: values.apiUrl,
    username: values.username,
    password: values.password,
    connId: values.name,
    test: false,
  })
  router.back()
})

const testConnection = () => {
  connectionCheck.value = ConnectionCheck.IN_PROGRESS
  const name = `test-connection-${Date.now()}`

  const disconnectTest = (name: string) => {
    invoke('gateway_disconnect', {
      connId: name,
    })
    connectionStore.connections.delete(name)
  }

  listen<MqttConnectionEvent>('mqtt-connected', (data) => {
    if (data.payload.id === name) {
      connectionCheck.value = ConnectionCheck.SUCCESS
      disconnectTest(name)
    }
  })

  listen<MqttConnectionEvent>('mqtt-disconnected', (data) => {
    if (data.payload.id === name) {
      if (connectionCheck.value === ConnectionCheck.IN_PROGRESS) {
        connectionCheck.value = ConnectionCheck.ERROR
        disconnectTest(name)
      }
    }
  })

  invoke('gateway_connect', {
    url: form.values.mqttUrl,
    username: form.values.username,
    password: form.values.password,
    connId: name,
    test: true,
  })
}

</script>

<template>
  <form @submit="onSubmit" class="max-w-xl justify-self-center align-middle content-center self-center">
    <h2
        class="heading-2"
    >New connection</h2>
    <div class="grid gap-4 pt-4">
      <FormField v-slot="{ componentField }" name="name">
        <FormItem>
          <FormLabel>Connection name</FormLabel>
          <FormControl>
            <Input type="text" v-bind="componentField"/>
          </FormControl>
          <FormMessage/>
        </FormItem>
      </FormField>
    </div>
    <div class="grid grid-cols-2 gap-4 py-4 items-start">
      <FormField v-slot="{ componentField }" name="mqttUrl">
        <FormItem>
          <FormLabel>MQTT URL</FormLabel>
          <FormControl>
            <Input type="text" placeholder="mqtt://127.0.0.1:1883" v-bind="componentField"/>
          </FormControl>
          <FormMessage/>
        </FormItem>
      </FormField>
      <FormField v-slot="{ componentField }" name="apiUrl">
        <FormItem>
          <FormLabel>API URL</FormLabel>
          <FormControl>
            <Input type="text" placeholder="http://127.0.0.1:8080" v-bind="componentField"/>
          </FormControl>
          <FormMessage/>
        </FormItem>
      </FormField>
    </div>
    <div class="flex flex-col gap-4 pb-4">
      <FormField v-slot="{ componentField }" name="username">
        <FormItem>
          <FormLabel>Username</FormLabel>
          <FormControl>
            <Input type="text" placeholder="admin" v-bind="componentField"/>
          </FormControl>
          <FormMessage/>
        </FormItem>
      </FormField>
      <FormField v-slot="{ componentField }" name="password">
        <FormItem>
          <FormLabel>Password</FormLabel>
          <FormControl>
            <Input type="password" v-bind="componentField"/>
          </FormControl>
          <FormMessage/>
        </FormItem>
      </FormField>
    </div>
    <div class="flex gap-4 justify-between">
      <div class="flex gap-2 items-center">
        <p v-if="connectionCheck == ConnectionCheck.IDLE"/>
        <p v-else-if="connectionCheck == ConnectionCheck.IN_PROGRESS" class="flex items-center gap-2">
          <Icon icon="heroicons-outline:refresh" class="w-4"/>
          Testing connection...
        </p>
        <p v-else-if="connectionCheck == ConnectionCheck.SUCCESS" class="flex items-center gap-2 text-positive">
          <Icon icon="heroicons-outline:check" class="w-4"/>
          Connection successful
        </p>
        <p v-else-if="connectionCheck == ConnectionCheck.ERROR" class="flex items-center gap-2 text-destructive">
          <Icon icon="heroicons-outline:x"/>
          Connection failed
        </p>
      </div>
      <div class="flex gap-2">
        <Button type="submit" variant="secondary">
          <Icon icon="heroicons-outline:check"/>
          <span>Save</span>
        </Button>
        <Button variant="secondary" @click.prevent="testConnection">
          <Icon icon="heroicons-outline:signal"/>
          <span>Test connection</span>
        </Button>
        <Button variant="destructive" @click.prevent="router.back()">
          <Icon icon="heroicons-outline:x"/>
          <span>Cancel</span>
        </Button>
      </div>
    </div>
  </form>
</template>