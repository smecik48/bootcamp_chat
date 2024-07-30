<script lang="ts">
import { ref } from 'vue';
import { bootcamp_chat_backend, canisterId, createActor } from '../../declarations/bootcamp_chat_backend';
import { AuthClient } from '@dfinity/auth-client';
import { HttpAgent } from '@dfinity/agent';
import type { Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';

export default {
  data() {
    return {
      newChat: "",
      chats: [] as string[][],
      identity: undefined as undefined | Identity,
      principal: undefined as undefined | Principal,
      targetPrincipal: ""
    }
  },
  methods: {
    isUserLogged() {
      if (!this.identity || !this.principal || this.principal === Principal.anonymous()){
        throw new Error("Log in man")
      }
      return{
        identity: this.identity,
        principal: this.principal
      }
    },
    validateTargetPrincipal(){
      
      if (this.targetPrincipal === ""){
        throw new Error("Principal not given")
      }

      const targetPrincipal = Principal.fromText(this.targetPrincipal.trim())
      if (!targetPrincipal || targetPrincipal === Principal.anonymous()){
        throw new Error("Wrong target")
      }
      return targetPrincipal
    },
    async dodajChat() {
      this.isUserLogged()
      const targetPrincipal = this.validateTargetPrincipal()

      const backend = createActor(canisterId, {
        agentOptions: {
          identity: this.identity
        }
      })

      await backend.add_chat_msg(this.newChat, targetPrincipal)
      
      await this.pobierzChaty()
    },
    async pobierzChaty() {
      const {identity, principal} = this.isUserLogged();
      const targetPrincipal = this.validateTargetPrincipal()

      const chatPath = [identity.getPrincipal(), targetPrincipal].sort()
      this.chats = await bootcamp_chat_backend.get_chat(chatPath)
    },
    async login() {
      const authClient = await AuthClient.create();
      await authClient.login({
        identityProvider: "http://ajuq4-ruaaa-aaaaa-qaaga-cai.localhost:4943",
        onSuccess: async () => {
          const identity = authClient.getIdentity();
          this.principal = identity.getPrincipal()
          console.log("Zalogowano", identity.getPrincipal())
          this.identity = identity;
          await this.pobierzChaty();
        }
      })

      
      
    }
  }
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    {{ principal }} <button @click="login">login</button>
    <div>
    <div>
      <input v-model="targetPrincipal" /> <button @click="pobierzChaty">pobierz chaty</button>
    </div>
    <div v-for="note in chats[0]">
      {{ note }}
    </div>
    </div>
    <div>
      <textarea v-model="newChat"></textarea><button @click="dodajChat">Dodaj notatke</button>
    </div>
  </main>
</template>