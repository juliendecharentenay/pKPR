<template>
  <div id="app">
    <Error :error="error.e" :error_message="error.msg" v-if="error != null" />

    <div v-if="show_panel_password()">
      <div class="mt-4 ml-2 mr-2 flex flex-col">
        <div class="flex-auto justify-self-center">
            <label for="db-password" class="block text-sm font-medium text-gray-700">Enter password for {{ database_filename }} </label>
            <div class="mt-1">
              <input v-model="password" type="password" name="db-password" id="db-password" class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md">
            </div>
        </div>

        <div class="mt-6 ml-2">
          <button type="button" class="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                @click="open_database()">
            Open
          </button>
        </div>
      </div>
    </div>

    <div v-if="show_panel_entry_list()">
      <div class="mt-4 ml-2 mr-2">
        <h3 class="text-lg leading-6 font-medium text-gray-900">
          {{ database_filename }} [{{ entry_list.length }}]
        </h3>

        <div class="justify-self-center">
          <label for="search" class="block text-sm font-medium text-gray-700">Search {{ p_filter }}</label>
          <div class="mt-1">
            <input v-model="filter" type="text" name="search" id="search" class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md">
          </div>
        </div>

       <div class="mt-5 ml-4 mr-4 border-t border-gray-200" v-if="entries.length > 0">
         <dl class="divide-y divide-gray-200">
           <div v-for="e in entries"
              :key="e.key"
              class="py-1 sm:py-2 sm:grid sm:grid-cols-3 sm:gap-4"
           >
             <dt class="test-sm font-medium text-gray-500">{{ e.title }}</dt>
             <dd class="mt-1 flex text-sm text-gray-900 sm:mt-0 sm:col-span-2">
               <span class="flex-grow">{{ e.username }}</span>
               <span class="mr-4 flex-shrink-0">
                 <button type="button" 
                         class="bg-white rounded-md font-medium text indigo-600 hover:text-indigo-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                         @click="entry = e;"
                   >
                   View
                 </button>
               </span>
             </dd>
           </div>
         </dl>
       </div>
      </div>
    </div>

    <div v-if="show_panel_entry()">
      <div class="mt-4 ml-2 mr-2">
        <h3 class="text-lg leading-6 font-medium text-gray-900">
          <a href="#" @click="entry = null">{{ database_filename }}</a> / {{ entry.title }}
        </h3>
        <div class="mt-5 border-t border-gray-500">
          <dl class="divide-y divide-gray-200">

            <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4">
              <dt class="text-sm font-medium text-gray-500">Username</dt>
              <dd class="mt-1 flex text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                <span class="flex-grow">{{ entry.username }}</span>
                <span class="mr-4 flex-shrink-0">
                  <button type="button" 
                       class="bg-white rounded-md font-medium text indigo-600 hover:text-indigo-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                       @click="copy_to_clipboard(entry.username);"
                       >
                    Copy
                  </button>
                </span>
              </dd>
            </div>

            <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4">
              <dt class="text-sm font-medium text-gray-500">Password</dt>
              <dd class="mt-1 flex text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                <span class="flex-grow">****</span>
                <span class="mr-4 flex-shrink-0">
                  <button type="button" 
                       class="bg-white rounded-md font-medium text indigo-600 hover:text-indigo-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                       @click="copy_to_clipboard(entry.password);"
                       >
                    Copy
                  </button>
                </span>
              </dd>
            </div>

            <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4">
              <dt class="text-sm font-medium text-gray-500">URL</dt>
              <dd class="mt-1 flex text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                <span class="flex-grow">{{ entry.url }} </span>
                <span class="mr-4 flex-shrink-0">
                  <button type="button" 
                       class="bg-white rounded-md font-medium text indigo-600 hover:text-indigo-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                       @click="copy_to_clipboard(entry.url);"
                       >
                    Copy
                  </button>
                </span>
              </dd>
            </div>

            <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4">
              <dt class="text-sm font-medium text-gray-500">Notes</dt>
              <dd class="mt-1 flex text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                <span class="flex-grow">{{ entry.notes }} </span>
              </dd>
            </div>

          </dl>
        </div>
      </div>
    </div>

  </div>
</template>

<script>
import Error from "@/components/error";

export default {
  name: "App",
  components: {
    Error
  },
  data: function() {
    return {
      error: null,
      database_filename: null,
      password: "",
      entry_list: null,
      entry: null,
      p_filter: "",
      filtered_entry_list: null
    };
  },
  computed: {
    filter: {
      get: function() {return this.p_filter;},
      set: function(v) {
        this.p_filter = v;
        const re = new RegExp(this.p_filter, "i");
        this.filtered_entry_list = 
          (this.filter.length < 2
            ? this.entry_list
            : this.entry_list.filter((e) => (e.title.match(re) || e.notes.match(re) || e.url.match(re)))); // includes(this.filter)));
      }
    },
    entries: function() { 
      return this.filtered_entry_list;
    }
  },
  mounted: function() {
    try {
      this.invoke("OnMounted");
    } catch (e) {
      this.handle_error("Error whilst initializing", e);
    }
  },
  methods: {
    copy_to_clipboard: function(s) { 
     try {
       this.copyTextToClipboard(s); 
     } catch (e) {
       this.handle_error("Error copy to clipboard", e);
     }
    },
    show_panel_password: function() {
      return ((this.error === null) && (this.database_filename !== null) && (this.entry_list === null) && (this.entry === null));
    },
    show_panel_entry_list: function() {
      return ((this.error === null) && (this.database_filename !== null) && (this.entry_list !== null) && (this.entry === null));
    },
    show_panel_entry: function() {
      return ((this.error === null) && (this.database_filename !== null) && (this.entry_list !== null) && (this.entry !== null));
    },
    invoke: function(type, o = {}) {
      try {
        external.invoke(JSON.stringify({...o, type}));
      } catch (e) {
        this.handle_error("Communication error", e);
      }
    },
    handle_error: function(msg, e) {
      this.error = {msg, e};
    },
    set_database_filename: function(filename) {
      this.database_filename = filename;
    },
    set_entry_list: function(obj) {
      this.entry_list = obj.map((e,i) => { return {...e, key: i};});
      this.entry_list
        .sort((x, y) => {
          if (x.title.toLowerCase() < y.title.toLowerCase()) {
            return -1;
          } else if (x.title.toLowerCase() > y.title.toLowerCase()) {
            return 1;
          } else {
            return 0;
          }
        }); 
      this.filtered_entry_list = this.entry_list;
    },
    open_database: function() {
      try {
        this.invoke("OpenDatabase", {password: this.password});
      } catch (e) {
        this.handle_error("Error when opening database", e);
      }
    },

    /* From https://stackoverflow.com/questions/400212/how-do-i-copy-to-the-clipboard-in-javascript
    */
    fallbackCopyTextToClipboard: function(text) {
      var textArea = document.createElement("textarea");
      textArea.value = text;
  
      // Avoid scrolling to bottom
      textArea.style.top = "0";
      textArea.style.left = "0";
      textArea.style.position = "fixed";
      textArea.style.opacity = "0";
      textArea.style.zIndex = "-1";

      document.body.appendChild(textArea);
      textArea.focus();
      textArea.select();

      try {
        document.execCommand('copy');
      } catch (err) {
        this.handle_error('Fallback: Oops, unable to copy', err);
      }

      document.body.removeChild(textArea);
    },

    copyTextToClipboard: function(text) {
      if (!navigator.clipboard) {
        this.fallbackCopyTextToClipboard(text);
        return;
      }
      navigator.clipboard.writeText(text)
      .catch((err) => {
        this.handle_error('Async: Could not copy text: ', err);
      });
    }
    /* End */

  }
};
</script>

