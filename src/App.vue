<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { onMounted, ref, computed } from "vue"
import { get_settings, get_projects_list } from "./lib/api"
import { IProjects } from "./types/Project"
import { ISettings } from "./types/Settings"
import { convertMarkdownToHTML } from './lib/markdown_converter';

var settings = ref<ISettings>(<any>{});
var projects = ref<IProjects>([]);
var selected_project_index = ref<number>(-1)

onMounted(async () => {
  settings.value = await get_settings()
  console.log('>>>', settings.value)

  projects.value = await get_projects_list()
  console.log('>>', projects.value)
})

function selectProject(index: number) {
  selected_project_index.value = index
  console.log('>>>>>', selected_project_index.value);
}

// добавьте после объявления переменных
const selected_project_class = computed(() => {
  return (index:number) => {
    return index === selected_project_index.value;
  };
});

const selected_project_description = computed(() => {
  if (projects.value[selected_project_index.value]) {
    return convertMarkdownToHTML(projects.value[selected_project_index.value].markdown)
  }
  return '';
});
</script>

<template>
  <div class="main">
        <div class="left">
          <div class="left-pane">
            <div class="left-pane_wrapper">
              <div class="toolbar-left">
                  
              </div>
              <div class="left-pane_panel">
                <v-toolbar
                  dense
                  floating
                >
                  <v-text-field
                    hide-details
                    single-line
                  ></v-text-field>

                  <v-btn icon>
                    <v-icon>mdi-dots-vertical</v-icon>
                  </v-btn>
                </v-toolbar>
                <div class="list_wrapper">
                  <v-list lines="two">
                    <v-list-item
                      v-for="(project, index) in projects"
                      :key="index"
                      :subtitle="project.title"
                      :title="project.name"
                      :active="selected_project_class(index)"
                      @click="selectProject(index)"
                    >
                      <template v-slot:prepend>
                        <v-avatar color="grey-lighten-1">
                          <v-icon color="white">mdi-folder</v-icon>
                        </v-avatar>
                      </template>

                      <!--template v-slot:append>
                        <v-btn
                          color="grey-lighten-1"
                          icon="mdi-information"
                          variant="text"
                        ></v-btn>
                      </template-->
                    </v-list-item>
                  </v-list>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="right">
          <iframe
            :srcdoc="selected_project_description"
            frameborder="0"
            width="100%"
            height="100%"
          ></iframe>
        </div>
  </div>
          <!--
  <div class="main">
    <div class="left-list">
      <div class="left-list-bar">
      </div>
      <div class="left-list_content-scroll">
        <div class="list-group">

          <template v-for="(project, index) in projects" v-bind:key="index">
            <button 
              type="button" 
              :class="'list-group-item list-group-item-action ' + selected_project_class(index)"
              @click="selectProject(index)"
            >
              <div class="ms-2 me-auto">
                <div class="fw-bold">{{ project.title }}</div>
                <small>{{ project.path }}</small>
                {{ project.short_description }}
              </div>
            </button>
          </template>
        </div>
      </div>
    </div>
    <div class="right-panel">
      <div class="current-project-description" v-html="selected_project_description">
      </div>
    </div>
  </div>
          -->
</template>