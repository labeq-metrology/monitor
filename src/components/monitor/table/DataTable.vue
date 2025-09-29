<script setup lang="ts" generic="TData, TValue">
import {
  ColumnDef,
  FlexRender,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  useVueTable
} from '@tanstack/vue-table'

import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow,} from '@/components/ui/table'
import {Button} from "@/components/ui/button";
import {Input} from "@/components/ui/input";

const props = defineProps<{
  columns: ColumnDef<TData, TValue>[]
  data: TData[]
}>()

const table = useVueTable({
  get data() {
    return props.data
  },
  get columns() {
    return props.columns
  },
  getCoreRowModel: getCoreRowModel(),
  getPaginationRowModel: getPaginationRowModel(),
  getFilteredRowModel: getFilteredRowModel(),
  globalFilterFn: 'includesString'
})
</script>

<template>
  <div>
    <div class="flex items-center py-4">
      <Input class="max-w-sm" placeholder="Filter..."
             @update:model-value=" table.setGlobalFilter($event)" />
    </div>
    <div class="border rounded-md">
      <Table>
        <TableHeader>
          <TableRow v-for="headerGroup in table.getHeaderGroups()" :key="headerGroup.id">
            <TableHead class="h-7" v-for="header in headerGroup.headers" :key="header.id">
              <FlexRender
                  v-if="!header.isPlaceholder" :render="header.column.columnDef.header"
                  :props="header.getContext()"
              />
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <template v-if="table.getRowModel().rows?.length">
            <TableRow
                v-for="row in table.getRowModel().rows" :key="row.id"
                :data-state="row.getIsSelected() ? 'selected' : undefined"
            >
              <TableCell v-for="cell in row.getVisibleCells()" :key="cell.id">
                <FlexRender :render="cell.column.columnDef.cell" :props="cell.getContext()"/>
              </TableCell>
            </TableRow>
          </template>
          <template v-else>
            <TableRow>
              <TableCell :colspan="columns.length" class="text-center">
                No results.
              </TableCell>
            </TableRow>
          </template>
        </TableBody>
      </Table>
    </div>
    <div class="flex items-center justify-end py-4 space-x-2">
      <Button
          variant="outline"
          size="sm"
          :disabled="!table.getCanPreviousPage()"
          @click="table.previousPage()"
      >
        Previous
      </Button>
      <Button
          variant="outline"
          size="sm"
          :disabled="!table.getCanNextPage()"
          @click="table.nextPage()"
      >
        Next
      </Button>
    </div>
  </div>
</template>