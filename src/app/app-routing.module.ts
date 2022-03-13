import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

const routes: Routes = [
  { path: '', pathMatch: 'full', redirectTo: '/home' },
  { path: 'home', loadChildren: () => import('./pages/home/home.module').then(m => m.HomeModule) },
  { path: 'about', loadChildren: () => import('./pages/about/about.module').then(m => m.AboutModule) },
  { path: 'at-caseware', loadChildren: () => import('./pages/caseware/caseware.module').then(m => m.CasewareModule) },
  { path: 'documents', loadChildren: () => import('./pages/documents/documents.module').then(m => m.DocumentsModule) },
  { path: 'project-learnera', loadChildren: () => import('./pages/bfs/bfs.module').then(m => m.BfsModule) },
  { path: 'contact', loadChildren: () => import('./pages/contact/contact.module').then(m => m.ContactModule) }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
